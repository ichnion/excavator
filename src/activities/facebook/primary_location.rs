use rusqlite::{params, Connection};
use serde::Deserialize;
use uuid::Uuid;

#[rustfmt::skip]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct PrimaryLocation {
  pub primary_location: CityAndRegionAndZipcode
}

#[rustfmt::skip]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct CityAndRegionAndZipcode {
  pub city_region_pairs:  Vec<Vec<String>>,
  pub zipcode:  Vec<String>
}

#[allow(non_snake_case)]
impl PrimaryLocation {
    pub fn saveToDb(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        let my_uuid = Uuid::new_v4();

        for elem in self.primary_location.city_region_pairs.iter() {
            conn.execute(
                "INSERT INTO facebook_primary_location (
              uuid,
              city,
              region,
          )
          VALUES (?1, ?2, ?3)",
                params![&my_uuid.to_string(), &elem[0], &elem[1]],
            )
            .map_err(|err| println!("{:?}", err))
            .ok();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_primary_location() -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open("ichnion.db")?;
        let zip_code = "1510053".to_string();
        let city_region_zipcode = CityAndRegionAndZipcode {
            city_region_pairs: vec![vec!["Tokyo".to_string(), "Shibuya".to_string()]],
            zipcode: vec![zip_code],
        };
        let primary_location = PrimaryLocation {
            primary_location: city_region_zipcode,
        };
        let result = PrimaryLocation::saveToDb(&primary_location, &conn);
        assert_eq!(result, Ok(()));
        Ok(())
    }
}
