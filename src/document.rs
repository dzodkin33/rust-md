use crate::row::Row;

use std::fs;
use std::io::{Error, Write};


#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub filename: Option<String>
}

impl Document {
    pub fn read_file(filename:&str) -> Result<Self, std::io::Error> {
        let file_contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();

        for value in file_contents.lines() {
            rows.push(Row::from(value));
        }
        Ok(Self {
            rows,
            filename: Some(filename.to_string())
        })
    }

    pub fn get_row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

}