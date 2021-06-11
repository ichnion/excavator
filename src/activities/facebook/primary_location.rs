use rusqlite::{params, Connection};
use serde::Deserialize;
use uuid::Uuid;

#[rustfmt::skip]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct PrimaryLocation {
  pub primary_location_v2: Option<CityAndRegionAndZipcode>,
  pub primary_location: Option<CityAndRegionAndZipcode>
}

#[rustfmt::skip]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct CityAndRegionAndZipcode {
  // TODO: change data structure
  pub city_region_pairs:  Vec<Vec<String>>,
  pub zipcode:  Vec<String>
}

#[allow(non_snake_case)]
impl PrimaryLocation {
    pub fn saveToDb(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        let my_uuid = Uuid::new_v4();
        match &self.primary_location_v2 {
            None => None,
            Some(x) => Some(
                conn.execute(
                    "INSERT into facebook_primary_location
                                (uuid, city_region_pairs, zipcode)
                                values(?1, $2, $3)",
                    params![
                        &my_uuid.to_string(),
                        x.city_region_pairs[0][0],
                        x.zipcode[0],
                    ],
                )
                .map_err(|err| println!("{:?}", err))
                .ok(),
            ),
        };

        match &self.primary_location {
            None => None,
            Some(x) => Some(
                conn.execute(
                    "INSERT into facebook_primary_location
                                (uuid, city_region_pairs, zipcode)
                                values(?1, $2, $3)",
                    params![
                        &my_uuid.to_string(),
                        x.city_region_pairs[0][0],
                        x.zipcode[0],
                    ],
                )
                .map_err(|err| println!("{:?}", err))
                .ok(),
            ),
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_primary_location_v2() -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open("ichnion.db")?;
        let zip_code = "1510053".to_string();
        let city_region_zipcode = CityAndRegionAndZipcode {
            city_region_pairs: vec![vec!["Tokyo".to_string(), "Shibuya".to_string()]],
            zipcode: vec![zip_code],
        };
        let primary_location = PrimaryLocation {
            primary_location_v2: Some(city_region_zipcode),
            primary_location: None,
        };
        let result = PrimaryLocation::saveToDb(&primary_location, &conn);
        assert_eq!(result, Ok(()));
        Ok(())
    }
    #[test]
    fn test_primary_location() -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open("ichnion.db")?;
        let zip_code = "1510053".to_string();
        let city_region_zipcode = CityAndRegionAndZipcode {
            city_region_pairs: vec![vec!["Tokyo".to_string(), "Shibuya".to_string()]],
            zipcode: vec![zip_code],
        };
        let primary_location = PrimaryLocation {
            primary_location_v2: None,
            primary_location: Some(city_region_zipcode),
        };
        let result = PrimaryLocation::saveToDb(&primary_location, &conn);
        assert_eq!(result, Ok(()));
        Ok(())
    }
}
