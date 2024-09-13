#[macro_use]
extern crate rocket;

mod auth;
mod models;
mod repositories;
mod schema;

use auth::BasicAuth;
use diesel::result::Error::NotFound;
use models::{NewRustac, Rustac};
use repositories::RustacRepo;
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::response::status::{self, Custom};
use rocket::serde::json::{json, Json, Value};
use rocket::{Build, Rocket};
use rocket_sync_db_pools::database;

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

#[get("/rustacs")]
async fn get_rustacs(_auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        RustacRepo::find_multiple(c, 1000)
            .map(|rustacs| json!(rustacs))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[get("/rustacs/<id>")]
async fn view_rustac(id: i32, _auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustacRepo::find(c, id)
            .map(|rustacean| json!(rustacean))
            .map_err(|e| match e {
                NotFound => Custom(Status::NotFound, json!(e.to_string())),
                _ => Custom(Status::InternalServerError, json!(e.to_string())),
            })
    })
    .await
}

#[post("/rustacs", format = "json", data = "<new_rustac>")]
async fn create_rustac(
    _auth: BasicAuth,
    db: DbConn,
    new_rustac: Json<NewRustac>,
) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        RustacRepo::create(c, new_rustac.into_inner())
            .map(|rustac| json!(rustac))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[put("/rustacs/<id>", format = "json", data = "<rustac>")]
async fn update_rustac(
    id: i32,
    _auth: BasicAuth,
    db: DbConn,
    rustac: Json<Rustac>,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustacRepo::save(c, id, rustac.into_inner())
            .map(|rustac| json!(rustac))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[delete("/rustacs/<id>")]
async fn delete_rustac(
    id: i32,
    _auth: BasicAuth,
    db: DbConn,
) -> Result<status::NoContent, Custom<Value>> {
    db.run(move |c| {
        RustacRepo::find(c, id)
            .map(|_| status::NoContent)
            .map_err(|e| Custom(Status::NotFound, json!(e.to_string())))?;
        RustacRepo::delete(c, id)
            .map(|_| status::NoContent)
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

async fn run_db_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    DbConn::get_one(&rocket)
        .await
        .unwrap()
        .run(|c| {
            c.run_pending_migrations(MIGRATIONS).unwrap();
        })
        .await;

    rocket
}

#[catch(401)]
fn unauthorized() -> Value {
    json!("status: error -> Unauthorized access!!")
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not Found!")
}

#[catch(422)]
fn unprocessable() -> Value {
    json!("Invalid entity. Probably some missing fields?")
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                get_rustacs,
                view_rustac,
                create_rustac,
                update_rustac,
                delete_rustac
            ],
        )
        .register("/", catchers![not_found, unauthorized, unprocessable])
        .attach(DbConn::fairing())
        .attach(AdHoc::on_ignite("Diesel migrations", run_db_migrations))
        .launch()
        .await?;

    Ok(())
}
