
mod parser;
mod document;
mod row;
use std::env;


fn main() {
    let mut str = "* * *".to_string();
    str.retain(|c| !c.is_whitespace());
    
    println!("{}", contains_only(&str, '*'))
}

fn contains_only( row: &String, symbol: char) -> bool {
    let chars = row.chars();
    for char in chars {
            if char != symbol {
                return false;
            }
        }
    return true;
    }