use actix_session::{storage::CookieSessionStore, Session, SessionExt, SessionMiddleware};
use actix_web::cookie::Key;

use actix_web::{
    guard::{fn_guard, GuardContext, Not, Post},
    post, web, App, HttpResponse, HttpServer, Responder,
};
use std::collections::HashMap;

use strava_client::data_struct::{Config, User};
use strava_client::strava_client::StravaClient;
use tokio::sync::OnceCell;

static CLIENT: OnceCell<StravaClient> = OnceCell::const_new();
mod db_client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
    })
    .bind(("127.0.0.1", 8080))?
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
    return HttpResponse::Ok()
        .body(r#"{"last_modified":{"secs_since_epoch": 111, "nanos_since_epoch": 1}}"#);
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
                        r#"{{"last_modified":"succesfully logged in","logged_as":{}}}"#,
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
            return HttpResponse::InternalServerError().body(format!(
                r#"{{"message":"server error occurred during logging in"}}"#
            ));
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
