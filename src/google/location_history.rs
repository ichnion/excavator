use serde::Deserialize;
//use uuid::Uuid;
//use crate::db::*;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct LocationHistory {
    pub locations : Vec<Locations> 
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Locations {
    pub timestampMs      : String,
    pub latitudeE7       : i32,
    pub longitudeE7      : i32,
    pub accuracy         : u32,
    pub altitude         : Option<i32>,
    pub verticalAccuracy : Option<i32>,
    pub activity         : Option<Vec<Activity>>
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Activity {
    pub timestampMs : String,
    pub activity    : Vec<Activities>
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Activities {
    pub r#type     : String,
    pub confidence : i32
}

#[allow(non_snake_case)]
impl LocationHistory {
    pub fn saveToDb( &self ) {

        for elem in self.locations.iter() {
            let activity:String;        
            
            if let Some(act) = &elem.activity {
                activity = act[0].activity[0].r#type.to_string();
            }
            else {
                activity = "na".to_string(); 
            }
                
            println!("activity: {}, timestamp: {}, lat: {}, lng: {}",
                activity,
                elem.timestampMs.parse::<i64>().unwrap(),
                elem.latitudeE7  as f64/10000000.0,
                elem.longitudeE7 as f64/10000000.0);    
        }
    }    
}
