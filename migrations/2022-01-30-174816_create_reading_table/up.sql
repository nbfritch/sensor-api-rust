Create Table If Not Exists readings (
    id Serial Primary Key,
    sensor_id integer Not Null,
    read_at timestamp not null,
    reading numeric(4,3) not null,
    Constraint fk_sensor_id
        Foreign Key(sensor_id)
            References sensors(id)
)