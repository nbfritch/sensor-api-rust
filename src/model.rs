use crate::diesel::prelude::*;
use crate::rocket::serde::Serialize;
use crate::schema::{readings, sensors};

#[derive(Queryable, Identifiable, Serialize, Associations)]
#[serde(crate = "rocket::serde")]
pub struct Sensor {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Identifiable, Associations, Serialize)]
#[serde(crate = "rocket::serde")]
#[diesel(belongs_to(Sensor))]
pub struct Reading {
    pub id: i32,
    pub sensor_id: i32,
    pub read_at: String,
    pub reading: f32,
}
