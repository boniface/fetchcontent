use actix_web::{
    App, get, HttpResponse, HttpServer, post, Responder, web,
};

// This struct represents state
struct AppState {
    app_name: String,
}

async fn index() -> impl Responder {
    "Hello world!"
}

async fn php() -> impl Responder {
    " Look This Site Runs PhP"
}

async fn manage_state(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name; // <- get app_name


    format!("Hello {}!", app_name)
}

#[get("/")] // Handled By Service
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]  // Handled by Micros
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// handled by the Route
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body(" This is handled Manually  ")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                app_name: String::from("Actix-web"),
            })
            .service(hello)
            .route("/state", web::get().to(manage_state))
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .service(
                web::scope("/app")
                    // ...so this handles requests for `GET /app/index.html`
                    .route("/index.html", web::get().to(index))
                    .route("/access.php", web::get().to(php))
            )
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
