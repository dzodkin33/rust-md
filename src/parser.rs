
use std::error::Error;
use std::fs;
use std::io;

pub struct Parser {
}


impl Parser {

}

pub fn read_file(filename:String) -> Result<String, Box<dyn Error>> {
        let file_content = fs::read_to_string(filename)?;

        Ok(file_content)

    }

