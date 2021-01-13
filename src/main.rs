#[macro_use]
extern crate diesel;

//use crate::trackpoints::places::SavedPlace;
//use std::path::PathBuf;
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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Opt::from_args();

    let directory_name = &args.directory_name;

    for entry in WalkDir::new(directory_name)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let d_name = entry.path().to_string_lossy();

        if f_name.starts_with("MyActivity.json") {
            print!("processing {}", d_name);

            let rawdata = std::fs::read_to_string(&entry.path())?;

            let result: Vec<google::my_activity::MyActivity> = serde_json::from_str(&rawdata)?;

            for elem in result.iter() {
                elem.saveToDb();
            }
            println!("( {} records )", result.len());
        //let result: SavedPlace = serde_json::from_str(&rawdata)?;
        //println!("{:?}", result);
        } else if f_name.starts_with("Location History.json") {
            print!("processing {}", d_name);

            let rawdata = std::fs::read_to_string(&entry.path())?;

            let result: google::location_history::LocationHistory = serde_json::from_str(&rawdata)?;

            result.saveToDb();

            println!("( {} records )", result.locations.len());
        }
    }

    Ok(())
}
