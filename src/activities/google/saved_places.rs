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

#[allow(non_snake_case)]
impl SavedPlace {
    pub fn saveToDb(&self,conn: &Connection) -> Result<(), rusqlite::Error> {
        let my_uuid = Uuid::new_v4();

        for elem in self.features.iter() {
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

            Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_saved_places() -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open("ichnion.db")?;

        let _geometry = Geometry {
            coordinates: vec![0.1, 0.2, 0.3, 0.4],
            r#type: "".to_string(),
        };

        let coordinate = Coordinate {
            latitude: "35.681168".to_string(),
            longitude: "139.767059".to_string(),
        };

        let _location = Location {
            address: Some("Tokyo".to_string()),
            business_name: Some("Test".to_string()),
            country_code: Some("+81".to_string()),
            geo_coordinate: coordinate,
        };

        let property = Property {
            google_maps_url: "https://www.google.com/maps/dir/?api=1&test".to_string(),
            location: _location,
            published: "".to_string(),
            title: "title".to_string(),
            updated: "".to_string(),
        };

        let element = Element {
            geometry: _geometry,
            properties: property,
            r#type: "".to_string(),
        };

        let saved_place = SavedPlace {
            r#type: "".to_string(),
            features: vec![element],
        };

        let result = SavedPlace::saveToDb(&saved_place, &conn);

        assert_eq!(result, Ok(()));

        Ok(())
    }
}
