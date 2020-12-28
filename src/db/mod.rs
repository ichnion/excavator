use diesel::{prelude::*, sqlite::SqliteConnection};

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    let db = "./ichneos.db";
    SqliteConnection::establish(db)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db))
}

pub fn save_activity(connection: &SqliteConnection,
    uuid: &str, header: &str, title: &str,
    title_url: &str, time: &str) { 
    
    let task = models::MyActivityEntity { 
        uuid      : uuid.to_string(),
        header    : header.to_string(),
        title     : title.to_string(),
        title_url : title_url.to_string(),
        time      : time.to_string()
     };

    diesel::insert_into(schema::google_my_activity::table)
        .values(&task)
        .execute(connection)
        .expect("Error inserting new task");
}

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
        .expect("Error inserting new task");
}
