use crate::trackpoints::places::SavedPlace;
use structopt::StructOpt;
use walkdir::WalkDir;

mod trackpoints;

#[derive(Debug, StructOpt)]
#[structopt(name = "excavator")]
struct Opt {
    command: String,
    file_name: String,
    //#[structopt(parse(from_os_str))]
    //file: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Opt::from_args();
    let file_name = &args.file_name;

    for entry in WalkDir::new(".")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        if f_name.starts_with(file_name) {
            let rawdata = std::fs::read_to_string(&entry.path())?;
            let result: SavedPlace = serde_json::from_str(&rawdata)?;
            println!("{:?}", result);
        }
    }

    Ok(())
}
