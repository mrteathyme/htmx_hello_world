use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::fs::File;
use std::io::prelude::*;
use serde::Deserialize;

//Jank Ahoy!

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(load_html_string("index.html").await.unwrap())
}

#[derive(Deserialize)]
struct Parameters {
    timestamp: Option<f64>,
    close_count: Option<i32>
}

#[get("/hello_world")]
async fn hello(params: web::Query<Parameters>) -> impl Responder {
    let mut html = load_html_string("hello_world.html").await.unwrap();
    match params.timestamp {
        None => {
            html = html.replace("<{timestamp}>", &0.to_string())},
        Some(time) => html = html.replace("<{timestamp}>", &(time as i32).to_string())
    }
    match params.close_count {
        None => html = html.replace("<{close_count}>", &1.to_string()),
        _ => html = html.replace("<{close_count}>", &params.close_count.unwrap().to_string())
    }
    HttpResponse::Ok().body(html)
}

#[get("/close")]
async fn close(mut params: web::Query<Parameters>) -> impl Responder {
    let mut html = load_html_string("index.html").await.unwrap();
    let instructions = "<p style=\"margin-left:auto;margin-right:auto;width:fit-content;\" id=\"instructions\" hx-swap-oob=\"true\">{instruction}</p>"; 
    let instructions = match params.close_count {
        None | Some(1) => {params.close_count = Some(2); instructions.replace("{instruction}", "Try it again ;)")},
        _ => {params.close_count = Some(params.close_count.unwrap() + 1); instructions.replace("{instruction}", "HTML = State pog!")}
    };
    html = format!("{}\n{}", instructions, html);
    match params.timestamp {
       Some(time) => {
          html = html.replace("id=\"hello_world\"", &format!("id=\"hello_world\" hx-vals=\"js:{{timestamp: {}, close_count: {} }}\"", (time as i32).to_string(), params.close_count.unwrap() ));
       }
       _ => () 
    }
    HttpResponse::Ok().body(html)
}


async fn load_html_string(path: &str) -> std::io::Result<String> {
    let mut html_file = File::open(path)?;
    let mut html = String::new();
    html_file.read_to_string(&mut html)?;
    Ok(html)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(hello)
            .service(close)
    }).bind(("127.0.0.1",8080))?
    .run()
    .await
}
