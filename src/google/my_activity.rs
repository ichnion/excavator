use serde::Deserialize;
use uuid::Uuid;

use rusqlite::{Connection,params};

#[rustfmt::skip]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct MyActivity {
    pub header        : String,
    pub title         : String,
    pub subtitles     : Option<Vec<SubTitle>>,    
    pub titleUrl      : Option<String>,
    pub time          : String,
    pub products      : Vec<String>,
    pub details       : Option<Vec<Details>>,
    pub locationInfos : Option<Vec<LocationInfo>>
}

#[rustfmt::skip]
#[derive(Deserialize, Debug)]
pub struct LocationInfo {
    pub name   : String,
    pub url    : String,
    pub source : String
}

#[rustfmt::skip]
#[derive(Deserialize, Debug)]
pub struct Details {
    pub name   : String
}

#[rustfmt::skip]
#[derive(Deserialize, Debug)]
pub struct SubTitle {
    pub name   : String,
    pub url    : Option<String>
}

#[rustfmt::skip]
#[allow(non_snake_case)]
impl MyActivity {
    pub fn saveToDb(&self,conn: &Connection) {

        let my_uuid = Uuid::new_v4();
        let title_url = self.titleUrl.as_ref(); //clone();

        conn.execute("insert into google_my_activity (
                uuid,
                header,
                title,
                title_url,
                time
            )
            values (?1, ?2, ?3, ?4, ?5)", 
            params![
                &my_uuid.to_string(),
                &self.header,
                &self.title,
                &title_url.unwrap_or(&String::from("")).to_string(),
                &self.time,
            ]
        ).ok();

        if let Some(ref vec) = self.subtitles {
            for i in vec {
                conn.execute(
                    "insert into activity_sub_title 
                        (a_uuid, name, url) values (?1, ?2, ?3)", 
                    params![
                        &my_uuid.to_string(),
                        &i.name.to_string(),
                        &i.url.as_ref().unwrap_or(&String::from("")).to_string()
                    ]
                ).ok();                
            }
        }

        if let Some(ref vec) = self.locationInfos {
            for i in vec {
                conn.execute(
                    "insert into activity_location_info
                        (a_uuid, name, url, source)
                        values (?1, ?2, ?3, ?4)",
                    params![
                        &my_uuid.to_string(),
                        &i.name.to_string(),
                        &i.url.to_string(),
                        &i.source.to_string()
                    ]
                ).ok();                
            }
        }

        if let Some(ref vec) = self.details {
            for i in vec {
                conn.execute(
                    "insert into activity_details (a_uuid, name) 
                     values (?1, ?2)", 
                    params![
                        &my_uuid.to_string(), &i.name.to_string()
                    ]
                ).ok();
            }
        }

        for p in &self.products {
            conn.execute(
                "insert into activity_products (a_uuid,name) 
                 values (?1, ?2)",
                params![
                    &my_uuid.to_string(), &p.to_string()
                ]
            ).ok();
        }
    }
}
