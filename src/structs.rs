use serde::{Deserialize, Serialize};
use serde_json::Value;

/*
An .ipynb file is a json file in a format as following.

{
 "cells": [
  {
   "cell_type": "code",
   "execution_count": 5,
   "metadata": {},
   "outputs": [
    {
     "name": "stdout",
     "output_type": "stream",
     "text": vector of strings
    }
   ],
   "source": vector of strings
  },
  ...
 ],
 "metadata": {
  "kernelspec": {
   "display_name": "Python 3",
   "language": "python",
   "name": "python3"
  },
  "language_info": {
   "codemirror_mode": {
    "name": "ipython",
    "version": 3
   },
   ...
   "version": "3.7.7"
  }
 },
 "nbformat": 4,
 "nbformat_minor": 4
}
*/

#[derive(Serialize, Deserialize)]
struct KernelSpec {
    display_name: Option<String>,
    language: Option<String>,
    name: Option<String>,
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
    fn length(&self) -> usize {
        return self.source.len();
    }

    pub fn print(&self) {
        for (i, line) in self.source.iter().enumerate() {
            print!("{}| {}", i, line);
        }
        println!();
    }
    
    pub fn join(&self) -> String {
        // join all lines in source into one string
        let mut result = String::new();
        let digits = self.length().to_string().len();
        for (i, line) in self.source.iter().enumerate() {
            let s = format!("{:>2$}| {}", i, line, digits);
            result.push_str(&s);
        }
        return result
    }

    pub fn highlight(&self) -> String {
        use syntect::easy::HighlightLines;
        use syntect::parsing::SyntaxSet;
        use syntect::highlighting::{ThemeSet, Style};
        use syntect::util::as_24_bit_terminal_escaped;

        let mut result = String::new();
        
        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();

        // find proper syntax
        let syntax = {
            if self.cell_type == "code" {
                ps.find_syntax_by_extension("py").unwrap()
            }
            else if self.cell_type == "markdown" {
                ps.find_syntax_by_extension("md").unwrap()
            }
            else {
                ps.find_syntax_by_extension("txt").unwrap()
            }
        };
        
        let mut hl = HighlightLines::new(syntax, &ts.themes["InspiredGitHub"]);
        let digits = self.length().to_string().len();

        for (i, line) in self.source.iter().enumerate() {
            let ranges: Vec<(Style, &str)> = hl.highlight_line(line, &ps).unwrap();
            let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
            let s = format!("{:>2$}| {}", i, &escaped, digits);
            result.push_str(&s);
            result.push_str("\x1b[0m");  // clear formatting
        }
        return result
    }
}

#[derive(Serialize, Deserialize)]
pub struct MetaData {
    kernelspec: KernelSpec,
    language_info: LanguageInfo,
}

#[derive(Serialize, Deserialize)]
pub struct NoteBook {
    pub metadata: MetaData,
    pub nbformat: i32,
    pub nbformat_minor: i32,
    pub cells: Vec<Cell>,
}

impl NoteBook {
    pub fn print(&self) {
        let digits = self.cells.len().to_string().len();
        for (i, cell) in self.cells.iter().enumerate() {
            println!("---[{:>1$}]---------------------------------------------------------", i, digits);
            cell.print();
            println!("----------------------------------------------------------------\n");
        }
    }

    pub fn print_highlight(&self) {
        let digits = self.cells.len().to_string().len();
        for (i, cell) in self.cells.iter().enumerate() {
            println!("---[{:>1$}]---------------------------------------------------------", i, digits);
            println!("{}", cell.highlight());
            println!("-------------------------------------------------------------<{}>--\n", cell.cell_type);
        }
    }
}