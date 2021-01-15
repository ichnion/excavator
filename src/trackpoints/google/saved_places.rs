use serde::Deserialize;

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
