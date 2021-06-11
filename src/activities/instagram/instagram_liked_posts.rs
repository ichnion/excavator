use rusqlite::{params, Connection};
use serde::Deserialize;
use uuid::Uuid;

#[rustfmt::skip]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct LikedPosts {
  pub likes_media_likes: Vec<Likes>
}
#[rustfmt::skip]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Likes {
  pub title: String,
  pub string_list_data: Vec<Link>
}
#[rustfmt::skip]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Link {
  pub href: String, 
  pub timestamp: u64
}

#[allow(non_snake_case)]
impl LikedPosts {
    pub fn saveToDb(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        for elem in self.likes_media_likes.iter() {
            let my_uuid = Uuid::new_v4();
            for element in elem.string_list_data.iter() {
                let hrf = &element.href;
                let time = &element.timestamp.to_string();
                conn.execute(
                    "INSERT INTO instagram_liked_posts (
                        uuid,
                        timestamp,
                        publisher,
                        link
                    )
                    VALUES (?1, ?2, ?3, ?4)",
                    params![&my_uuid.to_string(), time, &elem.title, hrf,],
                )
                .map_err(|err| println!("{:?}", err))
                .ok();
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    #[test]
    fn test_instagram_liked_posts() -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open("ichnion.db")?;
        let LinkTest = Link {
            href: "Test_href".to_string(),
            timestamp: 10,
        };
        let LikesTest = Likes {
            title: "Test".to_string(),
            string_list_data: vec![LinkTest],
        };
        let like = LikedPosts {
            likes_media_likes: vec![LikesTest],
        };
        let result = LikedPosts::saveToDb(&like, &conn);

        assert_eq!(result, Ok(()));

        Ok(())
    }
}
