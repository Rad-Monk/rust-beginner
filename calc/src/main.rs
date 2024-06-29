use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("this is a calculator microservice")
}

#[get("/add/{a}/{b}")]
async fn add(info: web::Path<(i32, i32)>) -> impl Responder {
    let res = calc::add(info.0, info.1);
    HttpResponse::Ok().body(format!("{} + {} = {}", info.0, info.1, res))
}

#[get("/subtract/{a}/{b}")]
async fn subtract(info: web::Path<(i32, i32)>) -> impl Responder {
    let res = calc::sub(info.0, info.1);
    HttpResponse::Ok().body(format!("{} - {} = {}", info.0, info.1, res))
}

#[get("/multiply/{a}/{b}")]
async fn multiply(info: web::Path<(i32, i32)>) -> impl Responder {
    let res = calc::mul(info.0, info.1);
    HttpResponse::Ok().body(format!("{} x {} = {}", info.0, info.1, res))
}

#[get("/divide/{a}/{b}")]
async fn divide(info: web::Path<(i32, i32)>) -> impl Responder {
    let res = calc::div(info.0, info.1);
    HttpResponse::Ok().body(format!("{} / {} = {}", info.0, info.1, res))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(add)
            .service(subtract)
            .service(multiply)
            .service(divide)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
