

pub struct Token {
    token_type: String,
    content: String,
    markup: String,
    html_tag: String,
    //children: [Token]
    // todo:
    // block and hidden for markdown
}

impl Token {
    pub fn get_html_tag(&self) -> String {
        let tag  = &self.html_tag;
        tag.to_string()
    }

    pub fn get_content(&self) -> String {
        let string  = &self.content;
        string.to_string()
    }
}