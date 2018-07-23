#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_cors;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

extern crate r2d2;
extern crate r2d2_diesel;

use rocket_contrib::{Json, Value};

mod model { pub mod persona; }
use model::persona::{Persona};
mod db;
mod schema;

use rocket::http::Method;

use rocket_cors::{AllowedOrigins, AllowedHeaders};


/// Manually mount an OPTIONS route for your own handling
//#[options("/")]
//fn manual_options(cors: Guard) -> Responder<&str> {
//    cors.responder("Manual OPTIONS preflight handling")
//}

#[post("/", data = "<persona>")]
fn create(persona: Json<Persona>, connection: db::Connection) -> Json<Persona> {
    let insert = Persona { id: None, ..persona.into_inner() };
    Json(Persona::create(insert, &connection))
}

#[get("/")]
fn all(connection: db::Connection) -> Json<Value> {
    Json(json!(Persona::read(&connection)))
}

#[put("/<id>", data = "<persona>")]
fn update(id: i32, persona: Json<Persona>, connection: db::Connection) -> Json<Value> {
    let update = Persona { id: Some(id), ..persona.into_inner() };
    Json(json!({
        "success": Persona::update(id, update, &connection)
    }))
}


#[get("/<id>")]
fn readone(id: i32, connection: db::Connection) -> Json<Value> {
    Json(json!({
        "success": true,
        "persona": Persona::select(id, &connection)
    }))
}


#[delete("/<id>")]
fn delete(id: i32, connection: db::Connection) -> Json<Value> {
    Json(json!({
        "success": Persona::delete(id, &connection)
    }))
}

fn main() {
//    let (allowed_origins, failed_origins) = AllowedOrigins::some(&["http://localhost"]);
    let allowed_origins = AllowedOrigins::all();

    // You can also deserialize this
    let options = rocket_cors::Cors {
        allowed_origins: allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete, Method::Options].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    };
    rocket::ignite()
        .mount("/persona", routes![all,create, update, delete,readone])
        .attach(options)
        .manage(db::connect())
        .launch();
}