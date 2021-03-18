use rusqlite::{params, Connection};
use serde::Deserialize;
use uuid::Uuid;

#[rustfmt::skip]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct PrimaryPublicLocationObjects {
  pub city: String,
  pub region: String,
  pub country: String
}
#[rustfmt::skip]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct PrimaryPublicLocation {
  pub primary_public_location: PrimaryPublicLocationObjects
}

#[allow(non_snake_case)]
impl PrimaryPublicLocation {
    pub fn saveToDb(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        let my_uuid = Uuid::new_v4();
        conn.execute(
            "INSERT INTO facebook_primary_public_location (
            uuid,
            city,
            region,
            country
        )
        VALUES (?1, ?2, ?3, ?4)",
            params![
                &my_uuid.to_string(),
                &self.primary_public_location.city,
                &self.primary_public_location.region,
                &self.primary_public_location.country
            ],
        )
        .map_err(|err| println!("{:?}", err))
        .ok();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_primary_public_location() -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open("ichnion.db")?;
        let test_location = PrimaryPublicLocationObjects {
            city: "Tokyo".to_string(),
            region: "Tokyo".to_string(),
            country: "Japan".to_string(),
        };
        let primary_public_location = PrimaryPublicLocation {
            primary_public_location: test_location,
        };
        let result = PrimaryPublicLocation::saveToDb(&primary_public_location, &conn);
        assert!(result == Ok(()));
        Ok(())
    }
}
