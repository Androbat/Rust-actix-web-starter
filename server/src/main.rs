use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// Macro to denote the type of method and route
#[get("/")]
async fn hello()-> impl Responder {
    // HttpResponse is just a "struct" allowing to respond to the server,
    // which also contains the "status" and the requests parts such as "body"
    HttpResponse::Ok().body("Hello world")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/author")]
async fn own_test() -> impl Responder {
    let s = "Hermann Hesse".to_string();
    HttpResponse::Ok().body(s)
}

#[actix_web::main] // Defines that it is the entry point for the app
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new() // We are creating new service (handlers)
            .service(hello)
            .service(echo)
            .service(own_test)
            // That's how handlers are defined when they do not contain the macro #[method(/)]
            // .route("/", we::method().to(handler))
            .route("/hey", web::get().to(manual_hello)) 
    })
    // .bind("localhost", port) -> just define the entry point of the application
    .bind(("127.0.0.1", 8080))?
    .run() 
    .await // Since it is an async function we must await the call
}
