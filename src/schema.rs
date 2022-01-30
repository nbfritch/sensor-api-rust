table! {

    readings (id) {
        id -> Int4,
        sensor_id -> Int4,
        read_at -> Timestamp,
        reading -> Numeric,
    }
}

table! {
    sensors (id) {
        id -> Int4,
        name -> Varchar,
    }
}

joinable!(readings -> sensors (sensor_id));

allow_tables_to_appear_in_same_query!(readings, sensors,);
