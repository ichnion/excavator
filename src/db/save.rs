use crate::db::models;
use crate::db::schema;
use diesel::{prelude::*, sqlite::SqliteConnection};

pub fn establish_connection() -> SqliteConnection {
    let db = "./ichneos.db";
    SqliteConnection::establish(db).unwrap_or_else(|_| panic!("Error connecting to {}", db))
}

#[rustfmt::skip]
pub fn save_activity(connection: &SqliteConnection,
    uuid: &str, header: &str, title: &str,
    title_url: &str, time: &str) -> Result<usize, diesel::result::Error> {

    let task = models::MyActivityEntity {
        uuid      : uuid.to_string(),
        header    : header.to_string(),
        title     : title.to_string(),
        title_url : title_url.to_string(),
        time      : time.to_string()
     };

    let result = diesel::insert_into(schema::google_my_activity::table)
        .values(&task)
        .execute(connection);
     println!("result is: {:?}", result);
     result
}

#[rustfmt::skip]
pub fn save_sub_title(connection: &SqliteConnection,
    a_uuid: &str, name: &str, url: &str ) {

    let task = models::SubTitlesEntity {
        a_uuid  : a_uuid.to_string(),
        name    : name.to_string(),
        url     : url.to_string()
     };

    diesel::insert_into(schema::activity_sub_title::table)
        .values(&task)
        .execute(connection)
        .expect("Error inserting new activity_sub_title");
}

#[rustfmt::skip]
pub fn save_location_info(connection: &SqliteConnection,
    a_uuid: &str, name: &str, url: &str, source: &str ) {

    let task = models::LocationInfoEntity {
        a_uuid  : a_uuid.to_string(),
        name    : name.to_string(),
        url     : url.to_string(),
        source  : source.to_string()
     };

    diesel::insert_into(schema::activity_location_info::table)
        .values(&task)
        .execute(connection)
        .expect("Error inserting new activity_location_info");
}

#[rustfmt::skip]
pub fn save_products(connection: &SqliteConnection,
    a_uuid: &str, name: &str ) {

    let task = models::ProductsEntity {
        a_uuid  : a_uuid.to_string(),
        name    : name.to_string()
     };

    diesel::insert_into(schema::activity_products::table)
        .values(&task)
        .execute(connection)
        .expect("Error inserting new activity_products");
}

#[rustfmt::skip]
pub fn save_details(connection: &SqliteConnection,
    a_uuid: &str, name: &str ) {

    let task = models::DetailsEntity {
        a_uuid  : a_uuid.to_string(),
        name    : name.to_string()
     };

    diesel::insert_into(schema::activity_details::table)
        .values(&task)
        .execute(connection)
        .expect("Error inserting new activity_details");
}

#[rustfmt::skip]
#[cfg_attr(feature = "cargo-clippy", allow(clippy::too_many_arguments))]
pub fn save_location_history(connection: &SqliteConnection,
    activity: &str, p_timestamp_msec: i64,
    p_accuracy: i32, p_verticalaccuracy: i32,
    p_altitude: i32, p_lat: f32, p_lng: f32 ) {

    let task = models::LocationHistoryEntity {
        activity         : activity.to_string(),
        timestamp_msec   : p_timestamp_msec,
        accuracy         : p_accuracy,
        verticalaccuracy : p_verticalaccuracy,
        altitude         : p_altitude,
        lat              : p_lat,
        lng              : p_lng
     };

    diesel::insert_into(schema::location_history::table)
        .values(&task)
        .execute(connection)
        .expect("Error inserting new activity_details");
}

#[rustfmt::skip]
#[cfg_attr(feature = "cargo-clippy", allow(clippy::too_many_arguments))]
pub fn save_saved_place(connection: &SqliteConnection,
    activity: &str, p_timestamp_msec: i64,
    p_accuracy: i32, p_verticalaccuracy: i32,
    p_altitude: i32, p_lat: f32, p_lng: f32 ) {

    let task = models::LocationHistoryEntity {
        activity         : activity.to_string(),
        timestamp_msec   : p_timestamp_msec,
        accuracy         : p_accuracy,
        verticalaccuracy : p_verticalaccuracy,
        altitude         : p_altitude,
        lat              : p_lat,
        lng              : p_lng
     };

    diesel::insert_into(schema::location_history::table)
        .values(&task)
        .execute(connection)
        .expect("Error inserting new activity_details");
}
