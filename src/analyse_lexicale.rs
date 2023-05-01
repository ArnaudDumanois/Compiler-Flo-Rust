/*
Analyseur lexical du langage Flo
 */

use regex::Regex;

pub enum Token {
    // Identificateur et Entier
    Ident(String),
    Entier(i32),

    // opérateurs
    Plus,
    Moins,
    Mult,
    Div,
}

pub fn lexer(input: &str) -> Vec<Token> {

    let mut tokens = vec![];
    let regex_ident = Regex::new(r"^[a-zA-Z][a-zA-Z0-9]*$").unwrap();
    let regex_entier = Regex::new(r"^[0-9]+$").unwrap();
    let operateur = ['+', '-', '*', '/'];
    let regex_space = Regex::new(r"^\s+$").unwrap();

    let mut remaining_input = input;
    while !remaining_input.is_empty() {
        // ignorer les espaces
        /*
        if let Some(c) = remaining_input.chars().next() {
            if c.is_whitespace() {
                remaining_input = remaining_input.trim_start();
                continue;
            }
        }
         */
        if let Some(captures) = regex_space.captures(remaining_input) {
            remaining_input = &remaining_input[captures.get(0).unwrap().end()..];
            continue;
        }

        if let Some(captures) = regex_ident.captures(remaining_input) {
            let ident = captures.get(0).unwrap().as_str();
            tokens.push(Token::Ident(ident.to_string()));
            remaining_input = &remaining_input[captures.get(0).unwrap().end()..];
            continue;
        }

        if let Some(captures) = regex_entier.captures(remaining_input) {
            let entier = captures.get(0).unwrap().as_str();
            tokens.push(Token::Entier(entier.parse::<i32>().unwrap()));
            remaining_input = &remaining_input[captures.get(0).unwrap().end()..];
            continue;
        }

        // on vérifie si le caractère est un opérateur
        if operateur.contains(&remaining_input.chars().next().unwrap()) {
            match remaining_input.chars().next().unwrap() {
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Moins),
                '*' => tokens.push(Token::Mult),
                '/' => tokens.push(Token::Div),
                _ => panic!("Unexpected character: {}", remaining_input.chars().next().unwrap()),
            }
            remaining_input = &remaining_input[1..];
            continue;
        }

        panic!("Unexpected character: {}", remaining_input.chars().next().unwrap());
    }
    tokens
}

