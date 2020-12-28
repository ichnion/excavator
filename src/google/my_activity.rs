use serde::Deserialize;
use uuid::Uuid;

use crate::db::*;

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


#[derive(Deserialize, Debug)]
pub struct LocationInfo {
    pub name   : String,
    pub url    : String,
    pub source : String
}

#[derive(Deserialize, Debug)]
pub struct Details {
    pub name   : String
}

#[derive(Deserialize, Debug)]
pub struct SubTitle {
    pub name   : String,
    pub url    : Option<String>
}
/*
#[derive(Debug)]
pub struct MyActivityEntity {
    pub uuid      : String,
    pub header    : String,
    pub title     : String,
    pub title_url : String,
    pub time      : String
}

#[derive(Debug)]
pub struct SubTitlesEntity {
    pub a_uuid    : String,
    pub name      : String,
    pub url       : String
}
*/

#[allow(non_snake_case)]
impl MyActivity {
    
    pub fn saveToDb(&self) {
        
        let connection = establish_connection();    
         
        let my_uuid    = Uuid::new_v4();
        let title_url  = self.titleUrl.as_ref();//clone();
                    
        save_activity(
           &connection,
           &my_uuid.to_string(),
           &self.header,
           &self.title,
           &title_url.unwrap_or(&String::from("")).to_string(),
           &self.time  
        );
                
        if let Some(ref vec) = self.subtitles {                        
            for i in vec {
                save_sub_title(
                    &connection,
                    &my_uuid.to_string(),
                    &i.name.to_string(),
                    &i.url.as_ref().unwrap_or(&String::from("")).to_string()
                );
            }                             
        }

        for p in &self.products {
            println!("products: {}", p);
        }
    }
}