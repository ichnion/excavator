use serde::Deserialize;
use uuid::Uuid;

use rusqlite::{params, Connection};

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
    pub fn saveToDb(&self, conn: &Connection) -> Result<(), rusqlite::Error> {

        let my_uuid = Uuid::new_v4();
        let title_url = self.titleUrl.as_ref(); //clone();

        conn.execute("INSERT INTO google_my_activity (
                uuid,
                header,
                title,
                title_url,
                time
            )
            VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                &my_uuid.to_string(),
                &self.header,
                &self.title,
                &title_url.unwrap_or(&String::from("")).to_string(),
                &self.time,
            ]
        ).ok();

        if let Some(ref subtitles) = self.subtitles {
            let _ = subtitles.iter().map(|subtitle|
                conn.execute(
                    "INSERT INTO activity_sub_title
                        (a_uuid, name, url) values (?1, ?2, ?3)",
                    params![
                        &my_uuid.to_string(),
                        &subtitle.name.to_string(),
                        &subtitle.url.as_ref().unwrap_or(&String::from("")).to_string()
                    ]
                ).ok()
            );
        }

        if let Some(ref location_infos) = self.locationInfos {
            let _ = location_infos.iter().map(|location_info|
                conn.execute(
                    "INSERT INTO activity_location_info
                        (a_uuid, name, url, source)
                        VALUES (?1, ?2, ?3, ?4)",
                    params![
                        &my_uuid.to_string(),
                        &location_info.name.to_string(),
                        &location_info.url.to_string(),
                        &location_info.source.to_string()
                    ]
                ).ok()
            );
        }

        if let Some(ref details) = self.details {
            let _ = details.iter().map(|detail|
                conn.execute(
                    "INSERT INTO activity_details (a_uuid, name)
                     VALUES (?1, ?2)",
                    params![
                        &my_uuid.to_string(), &detail.name.to_string()
                    ]
                ).ok()
            );
        }

        let _ = &self.products.iter().map(|product|
            conn.execute(
                "INSERT INTO activity_products (a_uuid,name)
                 VALUES (?1, ?2)",
                params![
                    &my_uuid.to_string(), &product.to_string()
                ]
            ).ok()
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_my_activities() -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open("ichnion.db")?;

        let location_info = LocationInfo {
            name: "name".to_string(),
            url: "https://locationinfo.com".to_string(),
            source: "".to_string(),
        };

        let subtitle = SubTitle {
            name: "subtitle".to_string(),
            url: Some("https://subtitle.com".to_string()),
        };

        let detail = Details {
            name: "Test details".to_string(),
        };

        let my_activity = MyActivity {
            header: "test header".to_string(),
            title: "test title".to_string(),
            subtitles: Some(vec![subtitle]),
            titleUrl: Some("https://testtitle.com".to_string()),
            time: "2020/10/10/10:10".to_string(),
            products: vec!["test product".to_string()],
            details: Some(vec![detail]),
            locationInfos: Some(vec![location_info]),
        };

        let result = MyActivity::saveToDb(&my_activity, &conn);

        assert_eq!(result, Ok(()));

        Ok(())
    }
}
