use actix_web::{App, HttpServer, web, Responder, HttpResponse};
use std::env;
use sendgrid::v3::{Personalization, Email, Message, Content, Sender};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/mail", web::get().to(mail))
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

async fn mail() -> impl Responder {
    dotenv::dotenv().expect("Failed to read .env file");
    let api_key =  env::var("SG_API_KEY").expect("SG_API_KEY not found.");
    let email =  env::var("EMAIL").expect("EMAIL not found.");


    let p = Personalization::new()
        .add_to(Email::new().set_email(email.as_str()));

    let m = Message::new()
        .set_from(Email::new().set_email(email.as_str()))
        .set_subject("Subject")
        .add_content(
            Content::new()
                .set_content_type("text/html")
                .set_value("Test"),
        )
        .add_personalization(p);


    let sender = Sender::new(api_key);
    let response = sender.send(&m);
    match response {
        Ok(_ok) =>  HttpResponse::Ok().body("ok"),
        Err(_) => HttpResponse::BadRequest().body("error")
    }

}
