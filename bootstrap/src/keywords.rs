use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::token::Token;

pub static TOKEN_MAP: Lazy<HashMap<String, Token>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("fn".to_string(), Token::Function("fn"));
    map.insert("let".to_string(), Token::Assign("let"));
    map.insert("i32".to_string(), Token::Int("i32"));
    map
});
