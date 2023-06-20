
use std::fs;
use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;
use regex::Regex;
use walkdir::WalkDir;

#[derive(Debug, StructOpt)]
#[structopt(name = "remove_console_log", about = "A command-line tool to remove console.log statements from .ts files.")]
struct Opt {
    /// File path to clean
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let re = Regex::new(r"(?m)^\s*console.log.*$").unwrap();

    for entry in WalkDir::new(opt.path) {
        let entry = entry?;
        if entry.path().extension().map_or(false, |e| e == "ts") {
            let content = fs::read_to_string(entry.path())?;
            let cleaned = re.replace_all(&content, "").into_owned();
            fs::write(entry.path(), cleaned)?;
        }
    }
    Ok(())
}

