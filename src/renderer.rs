
use crate::token::Token;

pub fn render(token: Token) -> String {
    let tag = token.get_html_tag();
    let content = &mut token.get_content();

    let open_tag = format!("<{}>", tag);
    let closing_tag = format!("</{}>", tag);
    content.insert_str(0, &open_tag);
    content.push_str(&closing_tag);

    content.to_string()
}

