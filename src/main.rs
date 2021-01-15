#[macro_use]
extern crate diesel;

use structopt::StructOpt;
use trackpoints::google::{location_history, my_activity, saved_places};
use walkdir::WalkDir;

mod db;
mod trackpoints;

#[derive(Debug, StructOpt)]
#[structopt(name = "excavator")]
struct Opt {
    command: String,
    directory_name: String,
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
        let file_name = f_name.as_ref();
        match file_name {
            "MyActivity.json" => {
                print!("processing {}", d_name);

                let rawdata = std::fs::read_to_string(&entry.path())?;

                let result: Vec<my_activity::MyActivity> = serde_json::from_str(&rawdata)?;

                for elem in result.iter() {
                    elem.saveToDb();
                }
                println!("( {} records )", result.len());
            },
            "Location\tHistory.json" => {
                print!("processing {}", d_name);

                let rawdata = std::fs::read_to_string(&entry.path())?;

                let result: location_history::LocationHistory = serde_json::from_str(&rawdata)?;

                result.saveToDb();

                println!("( {} records )", result.locations.len());
            },
            "Saved Place.json" => {
                print!("processing {}", d_name);

                let rawdata = std::fs::read_to_string(&entry.path())?;

                let result: saved_places::SavedPlace = serde_json::from_str(&rawdata)?;

                //result.saveToDb();
            },
            _ => println!("No files are matched"),
        }
    }

    Ok(())
}
