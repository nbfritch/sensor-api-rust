#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::{database, diesel::PgConnection};

mod schema;
use self::schema::*;

mod model;
use model::Reading;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct OkResult {
    ok: bool,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ErrorResult {
    error: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
enum ApiResponse {
    Ok(OkResult),
    Error(ErrorResult),
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct NewReadingRequest {
    pub sensor_name: String,
    pub sensor_reading: f32,
    pub date: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct TemplateContext {
    title: String,
    message: String,
}

#[database("reading_database")]
struct ReadingDatabase(PgConnection);

#[get("/")]
fn index() -> Template {
    let temp_ctx = TemplateContext {
        title: "Temp Sensor thing".to_string(),
        message: "Boops".to_string(),
    };
    Template::render("index", &temp_ctx)
}

#[get("/api/readings?<timeperiod>")]
fn get_readings(db: ReadingDatabase, timeperiod: &str) -> Json<Vec<Reading>> {
    Json(vec![Reading {
        id: 1,
        sensor_id: 1,
        reading: 3.4,
        read_at: " ".to_string(),
    }])
}

//stub
#[post("/api/readings", data = "<reading>")]
async fn create_reading(
    db: ReadingDatabase,
    reading: Json<NewReadingRequest>,
) -> Json<ApiResponse> {
    let sensor = db
        .run(move |conn| {
            sensors::table
                .order(sensors::id.asc())
                .filter(sensors::name.eq(&reading.sensor_name))
                .select(sensors::all_columns)
                .first::<model::Sensor>(conn)
                .optional()
        })
        .await;
    match sensor {
        Result::Ok(Some(sen)) => Json(ApiResponse::Ok(OkResult { ok: true })),
        _ => Json(ApiResponse::Error(ErrorResult {
            error: "Could not find sensor".to_string(),
        })),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .attach(ReadingDatabase::fairing())
        .mount("/", routes![index, get_readings, create_reading])
}
