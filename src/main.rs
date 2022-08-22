pub mod structs;
use std::fs;
use crate::structs::NoteBook;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name="ipyprev", about="Preview an ipynb file.")]
struct Opt {
    file: String,
}

impl Opt {
    fn run() -> Result<(), Box<dyn std::error::Error>> {
        let opt = Self::from_args();
        let file = opt.file.clone();
        let contents: String = fs::read_to_string(&file)?.parse()?;
        let notebook: NoteBook = serde_json::from_str(contents.as_str())?;
        notebook.print();
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Opt::run()
}
