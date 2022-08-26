pub mod structs;
use std::fs;
use crate::structs::NoteBook;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name="ipyprev", about="Preview an ipynb file.")]
struct Opt {
    file: String,
    #[structopt(short, long)]
    plain: bool,
}

impl Opt {
    fn run() -> Result<(), Box<dyn std::error::Error>> {
        let opt = Self::from_args();
        let file = opt.file.clone();
        let plain = opt.plain;
        let contents: String = fs::read_to_string(&file)?.parse()?;
        let notebook: NoteBook = serde_json::from_str(contents.as_str())?;
        if plain {
            notebook.print();
        } else {
            notebook.print_highlight();
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Opt::run()
}
