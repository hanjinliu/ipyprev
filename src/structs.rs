use serde::{Deserialize, Serialize};
use serde_json::{Value};


#[derive(Serialize, Deserialize)]
struct KernelInfo {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
struct LanguageInfo {
    pub name: String,
    pub version: String,
    pub codemirror_mode: Value,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Cell {
    pub cell_type: String,
    pub execution_count: Option<u64>,
    pub metadata: Value,
    pub outputs: Option<Value>,
    pub source: Vec<String>,
}

impl Cell {
    pub fn print(&self) {
        for line in &self.source {
            print!("{}", line);
        }
        println!();
    }
}

#[derive(Serialize, Deserialize)]
struct MetaData {
    pub kernel_info: KernelInfo,
}

#[derive(Serialize, Deserialize)]
pub struct NoteBook {
    pub metadata: Value,
    pub nbformat: i32,
    pub nbformat_minor: i32,
    pub cells: Vec<Cell>,
}

impl NoteBook {
    pub fn print(&self) {
        for (i, cell) in self.cells.iter().enumerate() {
            println!("{} <{}>", i, cell.cell_type);
            cell.print();
            println!("");
        }
    }
}