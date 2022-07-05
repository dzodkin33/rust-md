pub struct Row {
    string: String
}



/// Original repo: https://github.com/pflenker/hecto-tutorial/blob/master/src/row.rs
impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string: String::from(slice),
        }
    }
}

impl Row {
    pub fn get_string(&self) -> String {
        let string  = &self.string;
        string.to_string()
    }
}