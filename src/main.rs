//use crate::trackpoints::places::SavedPlace;
//use std::path::PathBuf;

use rusqlite::{Connection, Result};

use structopt::StructOpt;
use walkdir::WalkDir;

mod db;
mod google;
mod trackpoints;

#[derive(Debug, StructOpt)]
#[structopt(name = "excavator")]
struct Opt {
    command: String,
    directory_name: String,
    //#[structopt(parse(from_os_str))]
    //file: PathBuf,
    #[structopt(short = "d", long = "dbfile", default_value = "ichnion.db")]
    dbfile: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Opt::from_args();

    let conn = Connection::open(&args.dbfile)?;
    db::schema::create_tables(&conn);


    let directory_name = &args.directory_name;

    for entry in WalkDir::new(directory_name)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let d_name = entry.path().to_string_lossy();

        if f_name.starts_with("MyActivity.json") || 
            f_name.starts_with("search-history.json") || 
            f_name.starts_with("watch-history.json") {
            
            print!("processing {}", d_name);

            let rawdata = std::fs::read_to_string(&entry.path())?;

            let result: Vec<google::my_activity::MyActivity> = serde_json::from_str(&rawdata)?;

            for elem in result.iter() {
                elem.saveToDb(&conn);
            }
            println!("( {} records )", result.len());
            
            //let result: SavedPlace = serde_json::from_str(&rawdata)?;
            //println!("{:?}", result);
        } else if f_name.starts_with("Location History.json") {
            
            print!("processing {}", d_name);

            let rawdata = std::fs::read_to_string(&entry.path())?;

            let result: google::location_history::LocationHistory = serde_json::from_str(&rawdata)?;

            result.saveToDb(&conn);

            println!("( {} records )", result.locations.len());
        } else if d_name.contains("Semantic Location History") && 
            f_name.ends_with(".json") {

            print!("processing {}", d_name);
            
            let rawdata = std::fs::read_to_string(&entry.path())?;
            let result: google::semantic_location_history::TimeLineObjects = serde_json::from_str(&rawdata)?;
            
            println!("( {} records )", result.timelineObjects.len());            
            result.saveToDb(&conn);
        }
    }

    println!("\n*** process completed ***");
    Ok(())
}
