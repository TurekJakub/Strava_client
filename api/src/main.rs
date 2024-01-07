use actix_session::{
    storage::CookieSessionStore, Session, SessionExt, SessionGetError, SessionMiddleware,
};
use actix_web::cookie::Key;
use tokio::sync::OnceCell;
use super::strava::StravaClient;
use actix_web::web::resource;
use actix_web::{get, guard::fn_guard, post, web, App, HttpResponse, HttpServer, Responder};
use mongodm::mongo::change_stream::session;
use std::time::SystemTime;

static CLIENT: OnceCell<StravaClient> = OnceCell::const_new();

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
                web::resource("/update_time").route(
                    web::route()
                        .guard(fn_guard(|x| {
                            match x.get_session().get::<String>("username") {
                                Ok(Some(username)) => {
                                    println!("username: {}", username);
                                    return true;
                                }
                                _ => {
                                    return false;
                                }
                            }
                        }))
                        .to(echo),
                ),
            )
            .service(login)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

//#[post("/update_time")]
async fn echo(req_body: String, session: Session) -> impl Responder {
    /*
        println!("{:?}", session.entries());
        match session.get::<String>("username") {
            Ok(Some(username)) => HttpResponse::Ok().body(format!(
                r#"{{"last_modified":{{"secs_since_epoch": 111, "nanos_since_epoch": 1}},"name":{}}}"#,
                username
            )),
            _ => {
                return HttpResponse::Unauthorized()
                .body(format!(r#"{{"message":"action forbiden"}}"#));
        }
    }
    */
    return HttpResponse::Ok().body(format!(
        r#"{{"last_modified":{{"secs_since_epoch": 111, "nanos_since_epoch": 1}},"name":{}}}"#,
        req_body
    ));
}
#[post("/login")]
async fn login(req_body: String, session: Session) -> impl Responder {
    match req_body.as_str() {
        r#""UwU""# => {
            session.renew();
            session.insert("username", "UwU").unwrap();
            return HttpResponse::Ok().body(format!(
                r#"{{"last_modified":{{"secs_since_epoch": 111, "nanos_since_epoch": 1}},"name":{}}}"#,
                req_body
            ));
        }
        _ => {
            return HttpResponse::Unauthorized().body(format!(
                r#"{{"message":"user not found","name":{}}}"#,
                req_body
            ));
        }
    }
}
