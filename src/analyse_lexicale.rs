/*
Analyseur lexical du langage Flo
 */

use regex::Regex;

#[derive(Debug)]
pub enum Op {
    Plus,
    Moins,
    Mult,
    Div,
}

#[derive(Debug)]
pub enum Token {
    // Identificateur et Entier
    Ident(String),
    Entier(i32),

    // opérateurs
    Operator(Op),
}


// Fonction lexer
pub fn lexer(mut code: &str) -> Vec<Token> {

    let regex_entier = Regex::new(r"^[0-9]+").unwrap();
    let regex_ident = Regex::new(r"^[a-zA-Z][a-zA-Z0-9]*").unwrap();

    let mut res = Vec::new();
    loop {
        code = code.trim_start();

        if code.is_empty() {
            break;
        }
        if let Some(m) = regex_entier.find(code) {
            res.push(Token::Entier(m.as_str().parse().unwrap()));
            code = &code[m.end()..];
        } else if let Some(m) = regex_ident.find(code) {
            res.push(Token::Ident(m.as_str().to_owned()));
            code = &code[m.end()..];
        } else {
            let c = code.chars().next().unwrap();
            match c {
                '+' => res.push(Token::Operator(Op::Plus)),
                '-' => res.push(Token::Operator(Op::Moins)),
                '*' => res.push(Token::Operator(Op::Mult)),
                '/' => res.push(Token::Operator(Op::Div)),
                _ => panic!("Caractère inconnu: {}", c)
            };
            code = &code[1..];
        }
    }
    res
}



















