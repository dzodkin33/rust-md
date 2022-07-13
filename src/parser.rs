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

    // TODO: employ logic to parse every word in a row.
    /// Parsers a row from  markdown formal to HTML format
    /// 
    /// Params: 
    ///     `self` - instance of parser
    ///     `index` - index of a row that is being parsed
    /// 
    /// 
    /// Returns:
    ///     Parsed line
    pub fn parse_row(&self ,index: usize) -> String {
        let row = self.input_file.get_row(index).unwrap().get_string();
        let row_by_word = row.split(" ").collect::<Vec<&str>>();
        let first_letter = row_by_word.get(0).unwrap().chars().next().unwrap();

        // ! looks terrible, it must be refactored later 
        let return_row = if first_letter == '#' {
            self.parse_header(row)
        } else if self.is_horizontal_rules(&row) {
            self.parse_horizontal_rules()
        } else {

            // TODO: test this out
            for word in row_by_word {
                let oldWord = word; 
                let newWord = self.prase_emphasis(&word.to_string());
                row.replace(word, &newWord);
            }
            row
        };
        return return_row;
    }

    /// Parsers a row from  markdown header  `# ...` to HTML row `<h1> ... </h1>`
    /// 
    /// Params: 
    ///     `self` - instance of parser,
    ///     `row` - a row that is getting parsed
    /// 
    /// 
    /// Returns:
    ///     Parsed line
    fn parse_header(&self, row: String)  -> String {
        let row_by_word = row.split(" ").collect::<Vec<&str>>();
        let header = row_by_word.get(0) .unwrap();

            if  !self.contains_only(&header.to_string(), '#') || header.len() > 6 {
                return row
            } else {
                let new_tag = format!("h{}", header.len());
                self.add_html_wrapper(&row, &new_tag, header.len()+1, row.len())
            }

    }

    /// Returns a html horizontal rulers
    fn parse_horizontal_rules(&self) -> String {
        "<hr />".to_string()
    }

    // ! Will be refactored later
    /// Checks if the string contains only a sertain character.
    /// 
    /// Params:
    /// `self` - instance of parser,
    /// `row` - checked row,
    /// `symbol` - a symbol that is searched for
    /// 
    /// Returns:
    /// `true` - if `row` contains only the `char`, otherwise - `false`
    fn contains_only(&self, row: &String, symbol: char) -> bool {
        let chars = row.chars();
        for char in chars {
                if char != symbol {
                    return false;
                }
            }
        return true;
    }


    /// Checks if markdown row is a horizontal rules.
    /// 
    /// Params: 
    /// `row` - row that is being checked 
    /// 
    /// Returns:
    /// `true` if a row is  a horizontal rules, otherwise - `false`
    fn is_horizontal_rules(&self, row: &String) -> bool {

        // removes whitespaces between characters so that the function 
        // could account for horizonal markdown syntx cases like `* * *` or `- - -` etc.
        let horizontal_rule = self.remove_whiespace(row);

        return horizontal_rule.len() >=  3 && (self.contains_only(&horizontal_rule, '*') || self.contains_only(&horizontal_rule, '-') || self.contains_only(&horizontal_rule, '_'));
    }

    /// Removes whitespaces between 
    /// 
    /// Params
    ///  `self`,
    ///  `row` - row that is getting modified
    /// 
    /// Returns
    ///   A modified row.
    fn remove_whiespace(&self, row: &String) -> String {
        let mut return_row = row.clone();
        return_row.retain(|c| !c.is_whitespace());
        return_row.to_string()
    }


    // TODO: add parsing for bold italic
    /// Find if the word is an emphasis and then wraps it into a correct html tag.
    /// 
    /// Params:
    /// 
    /// `self` - instance of a parser,
    /// `word` - a word that is getting parsed
    /// 
    /// Returns:
    /// 
    /// A parsed string wrapped into a correct wrap
    fn prase_emphasis(&self, word: &String) -> String{

        // ! Check if these bounds are actually true
        if self.is_bold(word) {
            let tag = "b";
            self.add_html_wrapper(word, tag, 3, word.len()-1)
            
        } else if self.is_italic(word) {
            let tag = "em";
            self.add_html_wrapper(word, tag, 2, word.len()-1)
        } else {
            word.clone().to_string()
        }

    }

    /// Find if the word is bold.
    /// 
    /// Params:
    /// 
    /// `self` - instance of a parser,
    /// `word` - a word that is getting checked
    fn is_bold(&self, word: &String) -> bool {
        self.is_double_wrapped(word, '*') || self.is_double_wrapped(word, '-') || self.is_double_wrapped(word, '_')
    }

    /// Find if the word is italic.
    /// 
    /// Params:
    /// 
    /// `self` - instance of a parser,
    /// `word` - a word that is getting checked
    fn is_italic(&self, word: &String) -> bool {
        self.is_wrapped(word, '*') || self.is_wrapped(word, '-') || self.is_wrapped(word, '_')
    }
    
    fn is_italic_bold(&self, word: & String) -> bool {
        self.is_triple_wrapped(word, '*') || self.is_wrapped(word, '-') || self.is_wrapped(word, '_') 
    }

    fn is_triple_wrapped(&self, word: &String, symbol: char) -> bool {
        let return_word = word.clone();
        let mut chars = return_word.chars();
        self.is_wrapped(word, symbol) && chars.nth(2) == Some(symbol) && chars.nth(word.len()-3) == Some(symbol)
    } 

    /// Find if the word is wrapped by two same symbols.
    /// 
    /// Params:
    /// 
    /// `self` - instance of a parser,
    /// `word` - a word that is getting checked
    fn is_double_wrapped(&self, word: &String, symbol: char) -> bool {
        let return_word = word.clone();
        let mut chars = return_word.chars();
        self.is_wrapped(word, symbol) && chars.nth(1) == Some(symbol) && chars.nth(word.len()-2) == Some(symbol)
    }

    /// Find if the word is wrapped by some symbol.
    /// 
    /// Params:
    /// 
    /// `self` - instance of a parser,
    /// `word` - a word that is getting checked
    fn is_wrapped(&self,  word: &String, symbol: char) -> bool {
        let return_word = word.clone();
        let mut chars = return_word.chars();
        chars.next() == Some(symbol) && chars.last().unwrap()  == symbol
    }

    /// Wrappes a string in html tags.
    /// 
    /// Params:
    /// 
    ///  `self`,
    ///  `row` - row being wrapped into an html tag,
    ///  `tag` - a tag that is used as a wrapper,
    ///  `tag_start` - where to insert opening tag,
    ///  `tag_end` - where to instert closing tag
    fn add_html_wrapper(&self, row: &String, tag: &str, tag_start: usize, tag_end: usize)  -> String {
        let return_word = row.clone();
        let new_row = &mut return_word[tag_start..tag_end].to_string();

        let open_tag = format!("<{}>", tag);
        let closing_tag = format!("</{}>", tag);
        new_row.insert_str(0, &open_tag);
        new_row.push_str(&closing_tag);

        return new_row.to_string()
    }

}

