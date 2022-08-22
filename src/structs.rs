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
    pub fn print(&self) {
        for line in &self.source {
            print!("{}", line);
        }
        println!();
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
        for (i, cell) in self.cells.iter().enumerate() {
            println!("{} <{}>", i, cell.cell_type);
            cell.print();
            println!("");
        }
    }
}