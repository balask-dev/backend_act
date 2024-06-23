use actix_web::{web::Data,App,post,HttpServer,HttpResponse,get,Responder};
use actix_cors::Cors;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn resp(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/post")]
async fn create(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

#[get("/")]
async fn init()-> impl Responder{
"hello"
}

#[actix_web::main]
async fn main()-> std::io::Result<()> {
    HttpServer::new(move || App::new()
    .service(hello))
    .bind(("127.0.0.1",4000))?
    .run()
    .await
}