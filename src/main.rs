// Made by Arsquid
// FOR GENERAL INFO ABOUT THE APP AND INSTRUCTIONS ON HOW TO USE IT, SEE README.md

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};


#[get("/")]
async fn index() -> impl Responder {

    HttpResponse::Ok().body("Website is under construction, please be patient!")

}

#[get("/test")]
async fn test() -> impl Responder {

    HttpResponse::Ok().body("Yay, it works!")

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    builder

    .set_private_key_file("/usr/src/a-t_3/ssl/key.pem", SslFiletype::PEM).unwrap();

    builder

    .set_certificate_chain_file("/usr/src/a-t_3/ssl/cert.pem").unwrap();



    HttpServer::new(|| App::new()
        
    .service(index)

    .service(test)
        
    )

    .bind_openssl("0.0.0.0:443", builder)?

    .run()

    .await

}
