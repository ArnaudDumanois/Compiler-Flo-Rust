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
    Mod,
    Inf,
    Sup,
    InfEg,
    SupEg,
    Aff,
    Eg,
    Diff,
}

#[derive(Debug)]
pub enum Lit {
    ParOuv,
    ParFerm,
    AccOuv,
    AccFerm,
    Virg,
    PointVirg,
}

#[derive(Debug)]
pub enum Booleen {
    Vrai,
    Faux,
}


#[derive(Debug)]
pub enum Token {
    // Identificateur et Entier
    Ident(String),
    Entier(i32),

    // opérateurs
    Operator(Op),

    // littéraux
    Litteral(Lit),

    // booléens
    Bool(Booleen),
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
            let ident = m.as_str().to_string();
            match ident.as_str() {
                "==" => res.push(Token::Operator(Op::Eg)),
                "!=" => res.push(Token::Operator(Op::Diff)),
                "<=" => res.push(Token::Operator(Op::InfEg)),
                ">=" => res.push(Token::Operator(Op::SupEg)),
                "si" => res.push(Token::Ident(ident)),
                "sinon" => res.push(Token::Ident(ident)),
                "fin" => res.push(Token::Ident(ident)),
                "tantque" => res.push(Token::Ident(ident)),
                "Vrai" => res.push(Token::Bool(Booleen::Vrai)),
                "Faux" => res.push(Token::Bool(Booleen::Faux)),
                _ => res.push(Token::Ident(ident)),
            };
            code = &code[m.end()..];
        } else {
            let c = code.chars().next().unwrap();
            match c {
                '+' => res.push(Token::Operator(Op::Plus)),
                '-' => res.push(Token::Operator(Op::Moins)),
                '*' => res.push(Token::Operator(Op::Mult)),
                '/' => res.push(Token::Operator(Op::Div)),
                '%' => res.push(Token::Operator(Op::Mod)),
                '<' => res.push(Token::Operator(Op::Inf)),
                '>' => res.push(Token::Operator(Op::Sup)),
                '=' => res.push(Token::Operator(Op::Aff)),
                '(' => res.push(Token::Litteral(Lit::ParOuv)),
                ')' => res.push(Token::Litteral(Lit::ParFerm)),
                '{' => res.push(Token::Litteral(Lit::AccOuv)),
                '}' => res.push(Token::Litteral(Lit::AccFerm)),
                ',' => res.push(Token::Litteral(Lit::Virg)),
                ';' => res.push(Token::Litteral(Lit::PointVirg)),
                _ => panic!("Caractère inconnu: {}", c)
            };
            code = &code[1..];
        }
    }
    res
}



















