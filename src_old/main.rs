#[macro_use]
extern crate log;

use listenfd::ListenFd;
// use std::env;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
// use sqlx::PgPool;
// use anyhow::Result;

// import todo module (routes and model)
// mod todo;

// default / handler
async fn index() -> impl Responder {
    HttpResponse::Ok().body(r#"
        Welcome to Actix-web with SQLx Todos example.
        Available routes:
        GET /todos -> list of all todos
        POST /todo -> create new todo, example: { "description": "learn actix and sqlx", "done": false }
        GET /todo/{id} -> show one todo with requested id
        PUT /todo/{id} -> update todo with requested id, example: { "description": "learn actix and sqlx", "done": true }
        DELETE /todo/{id} -> delete todo with requested id
    "#
    )
}

#[actix_rt::main]
async fn main() {
//     env_logger::init();

    // this will enable us to keep application running during recompile: systemfd --no-pid -s http::5000 -- cargo watch -x run
    let mut listenfd = ListenFd::from_env();

//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
//     let db_pool = PgPool::new(&database_url).await?;

    let mut server = HttpServer::new(move || {
        App::new()
//             .data(db_pool.clone()) // pass database pool to application so we can access it inside handlers
            .route("/", web::get().to(index))
//             .configure(todo::init) // init todo routes
    });

    server.bind(format!("{}:{}", "0.0.0.0", "8000");

    info!("Starting server");
    server.run().await;

    Ok(())
}