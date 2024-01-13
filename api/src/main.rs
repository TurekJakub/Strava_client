use actix_session::{storage::CookieSessionStore, Session, SessionExt, SessionMiddleware};
use actix_web::cookie::Key;

use actix_web::{
    guard::{fn_guard, Any, Get, GuardContext, Not, Post},
    post, web, App, HttpResponse, HttpServer, Responder,
};
use std::collections::HashMap;
use std::env;

use db_client::DbClient;
use strava_client::data_struct::{Config, SettingsRequest, User, UserDBEntry};
use strava_client::strava_client::StravaClient;
use tokio::sync::OnceCell;

static CLIENT: OnceCell<StravaClient> = OnceCell::const_new();
static DB_CLIENT: OnceCell<DbClient> = OnceCell::const_new();
mod db_client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let secret_key = Key::generate();
    HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_http_only(true)
                    .cookie_same_site(actix_web::cookie::SameSite::None)
                    .cookie_secure(false)
                    .build(),
            )
            .service(
                web::resource("/settings_update_time")
                    .route(
                        web::route()
                            .guard(Post())
                            .guard(fn_guard(route_guard))
                            .to(update_time),
                    )
                    .route(
                        web::route()
                            .guard(Not(Post()))
                            .to(|| HttpResponse::MethodNotAllowed()),
                    )
                    .default_service(web::route().to(unauthorized)),
            )
            .service(login)
            .service(
                web::resource("/logout")
                    .route(
                        web::route()
                            .guard(fn_guard(route_guard))
                            .guard(Post())
                            .to(logout),
                    )
                    .default_service(web::route().to(unauthorized)),
            )
            .service(
                web::resource("/user_menu")
                    .route(
                        web::route()
                            .guard(fn_guard(route_guard))
                            .guard(Post())
                            .to(get_user_menu),
                    )
                    .default_service(web::route().to(unauthorized)),
            )
            .service(
                web::resource("/user_settings")
                    .route(
                        web::post()
                            .guard(fn_guard(route_guard))
                            .to(set_user_settings),
                    )
                    .route(
                        web::get()
                            .guard(fn_guard(route_guard))
                            .to(get_user_settings),
                    )
                    .route(
                        web::route()
                            .guard(Any(Not(Get())).or(Not(Post())))
                            .to(|| HttpResponse::MethodNotAllowed()),
                    )
                    .default_service(web::route().to(unauthorized)),
            )
    })
    .bind((
        env::var("IP_ADRESS").unwrap(),
        env::var("PORT").unwrap().parse().unwrap(),
    ))?
    .run()
    .await
}

fn route_guard(x: &GuardContext) -> bool {
    match x.get_session().get::<String>("username") {
        Ok(Some(_)) => {
            return true;
        }
        _ => {
            return false;
        }
    }
}
async fn update_time() -> impl Responder {
    let time = DB_CLIENT
        .get_or_init(|| async { DbClient::new().await.unwrap() })
        .await
        .get_settings_update_time("test")
        .await;
    match time {
        Ok(time) => match time {
            Some(time) => succes(
                "settings_update_time",
                serde_json::to_string(&time).unwrap().as_str(),
            ),
            None => {
                return HttpResponse::NoContent().body(format!(
                    r#"{{"message":"no settings found for given user"}}"#,
                ));
            }
        },
        Err(_) => server_error("server error occurred while loading user data"),
    }
}
async fn get_user_menu() -> impl Responder {
    let menu = CLIENT.get().unwrap().get_menu().await.unwrap();
    return HttpResponse::Ok().body(format!(
        r#"{{"name":{}}}"#,
        serde_json::to_string(&menu).unwrap()
    ));
}
#[post("/login")]
async fn login(req_body: String, session: Session) -> impl Responder {
    match serde_json::from_str::<User<'_>>(&req_body) {
        Ok(user_data) => {
            let res = CLIENT
                .get_or_init(|| async {
                    StravaClient::new_with_settings(Config {
                        settings: HashMap::from([("data_source".to_owned(), "api".to_owned())]),
                    })
                    .await
                    .unwrap()
                })
                .await
                .login(&user_data)
                .await;
            match res {
                Ok(_) => {
                    session.renew();
                    session.insert("username", user_data.username).unwrap();
                    return HttpResponse::Ok().body(format!(
                        r#"{{"message":"succesfully logged in","logged_as":{}}}"#,
                        user_data.username
                    ));
                }
                Err(_) => {
                    return HttpResponse::Unauthorized()
                        .body(format!(r#"{{"message":"incorrect user credentials"}}"#));
                }
            }
        }
        Err(_) => {
            return server_error("server error occurred during logging in");
        }
    }
}
async fn get_user_settings(session: Session) -> impl Responder {
    let settings = DB_CLIENT
        .get_or_init(|| async { DbClient::new().await.unwrap() })
        .await
        .get_settings(session.get::<String>("username").unwrap().unwrap().as_str())
        .await;
    match settings {
        Ok(settings) => match settings {
            Some(settings) => {
                return succes(
                    "settings",
                    serde_json::to_string(&settings).unwrap().as_str(),
                );
            }
            None => {
                return HttpResponse::NoContent().body(format!(
                    r#"{{"settings":{{}}, "message":"no settings found for given user"}}"#,
                ));
            }
        },
        Err(_) => server_error("server error occurred while loading user data"),
    }
}
async fn set_user_settings(session: Session, req_body: String) -> impl Responder {
    let settings = serde_json::from_str::<SettingsRequest>(&req_body);
    match settings {
        Ok(settings) => {
            let settings = UserDBEntry {
                username: session.get::<String>("username").unwrap().unwrap(),
                settings: settings.settings,
                settings_update_time: settings.settings_update_time,
            };
            let res = DB_CLIENT
                .get_or_init(|| async { DbClient::new().await.unwrap() })
                .await
                .insert_user(settings)
                .await;
            match res {
                Ok(_) => succes("message", "new settings was succesfully saved"),
                Err(_) => {
                    server_error("server error occurred while saving user data, try it again later")
                }
            }
        }
        Err(_) => {
            return HttpResponse::BadRequest()
                .body(format!(r#"{{"message":"invalid request body"}}"#));
        }
    }
}
//#[post("/logout")]
async fn logout(session: Session) -> impl Responder {
    let username = session.get::<String>("username").unwrap().unwrap();
    session.purge();
    return HttpResponse::Ok().body(format!(r#"{{"status":"logged out","name":{}}}"#, username));
}
async fn unauthorized() -> impl Responder {
    return HttpResponse::Unauthorized().body(format!(r#"{{"message":"action forbiden"}}"#,));
}
fn server_error(message: &str) -> HttpResponse {
    return HttpResponse::InternalServerError().body(format!(r#"{{"message":"{}"}}"#, message));
}
fn succes(key: &str, value: &str) -> HttpResponse {
    return HttpResponse::Ok().body(format!(r#"{{"{}":"{}"}}"#, key, value));
}
