use serde::{Deserialize, Serialize};
use serde_json::Value;
use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::as_24_bit_terminal_escaped;
use crate::enums::Language;

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

#[derive(Serialize, Deserialize, Clone)]
pub struct KernelSpec {
    display_name: Option<String>,
    language: Option<String>,
    name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LanguageInfo {
    name: String,
    version: Option<String>,
    codemirror_mode: Option<Value>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Output {
    name: String,
    output_type: String,
    text: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Cell {
    cell_type: String,
    execution_count: Option<u64>,
    metadata: Value,  // NOTE: Not Metadata type!
    outputs: Option<Vec<Output>>,
    source: Vec<String>,
}

impl Cell {
    // number of lines in the cell
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

        for (i, line) in self.source.iter().enumerate() {
            let ranges: Vec<(Style, &str)> = hl.highlight_line(line, &ps).unwrap();
            let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
            let s = format!("{:>3}| {}", i + 1, &escaped);
            result.push_str(&s);
            result.push_str("\x1b[0m");  // clear formatting
        }
        return result
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MetaData {
    kernelspec: KernelSpec,
    language_info: LanguageInfo,
}

#[derive(Serialize, Deserialize)]
pub struct NoteBook {
    metadata: MetaData,
    nbformat: i32,
    nbformat_minor: i32,
    cells: Vec<Cell>,
}

impl NoteBook {
    const WIDTH: usize = 90;

    pub fn print(&self) {
        for (i, cell) in self.cells.iter().enumerate() {
            println!("---[{:>3}]---------------------------------------------------------", i + 1);
            cell.print();
            println!("----------------------------------------------------------------\n");
        }
    }

    pub fn print_highlight(&self) {
        for (i, cell) in self.cells.iter().enumerate() {
            let lang = match &*cell.cell_type {
                "code" => self.get_language().to_str().to_string(),
                _ => cell.cell_type.clone(),
            };
            let line0 = "-".to_string().repeat(NoteBook::WIDTH - 11);
            let lang_cap = capitalize(&lang);
            let line1 = "-".to_string().repeat(NoteBook::WIDTH - lang_cap.len() - 8);
            println!("---+{}[{:>3}]--", line0, i + 1);
            println!("{}", cell.highlight());
            println!("---+{}<{}>--\n", line1, lang_cap);

            // output
            match &cell.outputs {
                Some(outputs) => {
                    for output in outputs {
                        if output.output_type == "stream" {
                            match &output.text {
                                Some(text) => {
                                    for line in text.iter() {
                                        print!("{}", line);
                                    }
                                }
                                None => {}
                            }
                        }
                    }
                }
                None => {}
            }
        }
    }

    fn get_language(&self) -> Language {
        return match self.metadata.kernelspec.language {
            Some(ref lang) => Language::from_str(lang),
            None => Language::Unknown,
        }
    }
}

fn capitalize(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}