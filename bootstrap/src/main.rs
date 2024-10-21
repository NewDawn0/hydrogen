use bootstrap::lexer::Lexer;
fn main() {
    let code = r#"
        fn test_function() -> i32 {
            let a = 5;
            return a;
        }
    "#;
    let mut lex = Lexer::new(code);
    let mut tok = bootstrap::token::Token::Ident("");
    while {
        let (has_tok, stok) = lex.next_token();
        tok = stok;
        has_tok
    } {
        println!("Token: {{ {:?} }}", tok);
    }
    println!("No more tokens");
}
