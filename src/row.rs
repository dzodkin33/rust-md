use std::string;

pub struct Row {
    string: String,
    len: usize,
}



/// Original repo: https://github.com/pflenker/hecto-tutorial/blob/master/src/row.rs
impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        let string = String::from(slice);
        Self {
            string: String::from(slice),
            len: string.len(),
        }
    }
}

impl Row {
    pub fn get_string(&self) -> String {
        let string  = &self.string;
        string.to_string()
    }

    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}