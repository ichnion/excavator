use rusqlite::{params, Connection};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct Geometry {
    pub coordinates: Vec<f32>,
    pub r#type: String,
}

#[derive(Deserialize, Debug)]
pub struct Coordinate {
    #[serde(rename = "Latitude")]
    pub latitude: String,
    #[serde(rename = "Longitude")]
    pub longitude: String,
}

#[derive(Deserialize, Debug)]
pub struct Location {
    #[serde(rename = "Address")]
    pub address: Option<String>,
    #[serde(rename = "Business Name")]
    pub business_name: Option<String>,
    #[serde(rename = "Country Code")]
    pub country_code: Option<String>,
    #[serde(rename = "Geo Coordinates")]
    pub geo_coordinate: Coordinate,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Property {
    #[serde(rename = "Google Maps URL")]
    pub google_maps_url: String,
    pub location: Location,
    pub published: String,
    pub title: String,
    pub updated: String,
}

#[derive(Deserialize, Debug)]
pub struct Element {
    pub geometry: Geometry,
    pub properties: Property,
    pub r#type: String,
}

#[derive(Deserialize, Debug)]
pub struct SavedPlace {
    pub r#type: String,
    pub features: Vec<Element>,
}

#[rustfmt::skip]
#[allow(non_snake_case)]
impl SavedPlace {
    pub fn saveToDb(&self,conn: &Connection) {
        let my_uuid = Uuid::new_v4();

            for elem in &self.features {
                conn.execute(
                    "INSERT INTO google_my_activity (
                        uuid,
                        name,
                        address,
                        url,
                        lat,
                        lng
                    )
                    VALUES (?1, ?2, ?3, ?4, ?5)",
                    params![
                        &my_uuid.to_string(),
                        &elem.properties.location.business_name,
                        &elem.properties.location.address,
                        &elem.properties.google_maps_url,
                        &elem.properties.location.geo_coordinate.latitude,
                        &elem.properties.location.geo_coordinate.longitude,
                    ]
                ).ok();
            }
    }
}
