use std::sync::Arc;

#[macro_use]
extern crate juniper;

use actix_web::{web, App, HttpResponse, HttpServer, Responder, Error};
use futures::future::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use serde::Deserialize;

mod schema;
use crate::schema::{create_schema, Schema};

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Deserialize)]
struct Info {
    color: String,
    other: Option<String>,
}

fn index2((path, query): (web::Path<(String)>, web::Query<Info>)) -> impl Responder {
    let a = format!(
        "Hello world! path={} color={} other={}",
        path,
        query.color,
        match query.other {
            None => "No other query",
            Some(ref x) => x,
        }
    );
    return HttpResponse::Ok().body(a);
}

fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|user| {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(user))
    })
}



fn main() {

    std::env::set_var("RUST_LOG", "actix_web=info");
    // Create Juniper schema
    let schema = std::sync::Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource("/graphql").route(web::post().to_async(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
            .route("/", web::get().to(index))
            .route("/{name}", web::get().to(index2))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .unwrap();
}
