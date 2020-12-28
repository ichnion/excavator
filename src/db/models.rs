use super::schema::*;

#[derive(Insertable)]
#[table_name = "google_my_activity"]
pub struct MyActivityEntity {
    pub uuid      : String,
    pub header    : String,
    pub title     : String,
    pub title_url : String,
    pub time      : String
}

#[derive(Insertable)]
#[table_name = "activity_sub_title"]
pub struct SubTitlesEntity {
    pub a_uuid    : String,
    pub name      : String,
    pub url       : String
}

#[derive(Insertable)]
#[table_name = "activity_location_info"]
pub struct LocationInfoEntity {
    pub a_uuid    : String,
    pub name      : String,
    pub url       : String,
    pub source    : String
}

#[derive(Insertable)]
#[table_name = "activity_products"]
pub struct ProductsEntity {
    pub a_uuid    : String,
    pub name      : String
}

#[derive(Insertable)]
#[table_name = "activity_details"]
pub struct DetailsEntity {
    pub a_uuid    : String,
    pub name      : String
}
