use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, Error, HttpServer, dev::ServiceRequest};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::extractors::bearer::{self, BearerAuth};
use actix_web_httpauth::middleware::HttpAuthentication;
// use config::Config;
use env_logger::Env;

mod controller;
mod database;
mod schema;

async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let config = req.app_data::<bearer::Config>()
        .cloned()
        .unwrap_or_default()
        .scope("urn:flight-plans");

    match database::get_user_by_api_key(String::from(credentials.token())) {
        Ok(user) => {
            match user {
                Some(_) => {
                    return Ok(req);
                },
                None => {
                    Err((AuthenticationError::from(config).into(),req))
                },
            }
        },
        Err(_) => Err((AuthenticationError::from(config).into(),req))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let settings = Config::builder()
    //     .add_source(config::File::with_name("config"))
    //     .build()
    //     .unwrap();

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(move || {
        let middleware = HttpAuthentication::bearer(validator);
        App::new()
            .service(controller::new_user)
            .service(controller::get_all_flight_plans)
            .service(controller::file_flight_plan)
            .service(controller::get_flight_plan_by_id)
            .service(controller::delete_flight_plan_by_id)
            .service(controller::index)
            .wrap(middleware)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allow_any_method()
                    .allow_any_origin()
                    .max_age(3600),
            )
    })
    .bind(("0.0.0.0", 3000))?
    .workers(2)
    .run()
    .await
}
