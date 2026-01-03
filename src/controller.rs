use crate::database;
use crate::database::{
    delete_flight_plan, fetch_all_flight_plans, fetch_flight_plan_by_id, insert_flight_plan,
};
use crate::schema::{FlightPlan, User};
use actix_web::{HttpResponse, Responder, delete, get, post, web};

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("server running")
}

#[post("/api/v1/admin/user")]
pub async fn new_user(user: web::Json<User>) -> impl Responder {
    match database::create_user(user.into_inner()) {
        Ok(api_key) => HttpResponse::Ok().body(api_key),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/api/v1/flight_plans")]
pub async fn get_all_flight_plans() -> impl Responder {
    match fetch_all_flight_plans().unwrap() {
        Some(flight_plans) => HttpResponse::Ok()
            .content_type("application/json")
            .json(flight_plans),
        None => HttpResponse::NoContent().body("No flight plans found"),
    }
}

#[get("/api/v1/flight_plans/{flight_plan_id}")]
pub async fn get_flight_plan_by_id(path: web::Path<String>) -> impl Responder {
    let flight_plan_id = path.into_inner();
    let result = fetch_flight_plan_by_id(flight_plan_id.clone()).unwrap();
    match result {
        Some(flight_plan) => HttpResponse::Ok()
            .content_type("application/json")
            .json(flight_plan),
        None => HttpResponse::NotFound()
            .body(format!("Flight plan with ID {} not found", flight_plan_id)),
    }
}

#[delete("/api/v1/flight_plans/{flight_plan_id}")]
pub async fn delete_flight_plan_by_id(id: web::Path<String>) -> impl Responder {
    let flight_plan_id = id.into_inner();
    match delete_flight_plan(flight_plan_id.clone()) {
        Ok(success) => {
            if success {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::NotFound()
                    .body(format!("Flight plan with ID {} not found", flight_plan_id))
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete flight plan"),
    }
}

#[post("/api/v1/flightplan")]
pub async fn file_flight_plan(flight_plan: web::Json<FlightPlan>) -> impl Responder {
    match insert_flight_plan(flight_plan.into_inner()) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
