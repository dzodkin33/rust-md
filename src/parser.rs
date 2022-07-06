use crate::document::Document;
use crate::row::Row;

use std::env;
use std::error::Error;
use std::fs;
use std::io;


#[derive(Default)]
pub struct Parser {
    input_file: Document,
    //output_file: Document
}


impl Parser {

    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();


        // Varibale input_file is created and checked if there is a path in args.
        // Then the Document doc tries to read file on a path and if successful
        //  doc is assigned to input_file.
        let input_file = if let Some(file_name) = args.get(0) {
            let doc = Document::read_file(file_name);
            if let Ok(doc) = doc {
                doc
            } else {
                Document::default()
            }
        } else {
            Document::default()
        };

        Self {
            input_file
        }

    }

    /// Parsers a row from  markdown formal to HTML format
    /// 
    /// Params: 
    ///     * self - instance of parser
    ///     * index - index of a row that is being parsed
    /// 
    /// 
    /// Returns:
    ///     Parsed line
    /// 
    pub fn parse_row(&self ,index: usize) -> String {
        let row = self.input_file.get_row(index).unwrap().get_string();
        let row_by_word = row.split(" ").collect::<Vec<&str>>();
        let first_letter = row_by_word.get(0).unwrap().chars().next().unwrap();

        // ! looks terrible, it must be refactored later 
        let return_row = if first_letter == '#' {
            self.parse_header(row)
        } else {
            row
        };
        return return_row;
    }

    /// Parsers a row from  markdown header  `# ...` to HTML row `<h1> ... </h1>`
    /// 
    /// Params: 
    ///     * self - instance of parser
    ///     * row - a row that is getting parsed
    /// 
    /// 
    /// Returns:
    ///     Parsed line
    /// 
    fn parse_header(&self, row: String)  -> String {
        let row_by_word = row.split(" ").collect::<Vec<&str>>();

        let header = row_by_word.get(0) .unwrap();

            // Checks if the header is valid or not
            for char in header.chars() {
                if char != '#' {
                    return row;
                }
            }

            // max header in 6
            if header.len() > 6 {
                return row
            } else {
                let new_row = &mut row[header.len()+1..row.len()].to_string();

                let new_header = format!("<h{}>", header.len());
                new_row.insert_str(0, &new_header);

                let new_header = format!("</h{}>", header.len());

                new_row.push_str(&new_header);

                return new_row.to_string()
            }

    }
}