mod analyse_lexicale;
mod analyse_syntaxique;
mod arbre_abstrait;
mod table_symbole;
mod generation_code;

fn main() {

    let input = "1";
    let tokens = analyse_lexicale::lexer(input.trim());


    for token in tokens {
        match token {
            analyse_lexicale::Token::Entier(valeur) => println!("Entier: {}", valeur),
            analyse_lexicale::Token::Ident(ident) => println!("Ident: {}", ident),
            analyse_lexicale::Token::Plus => println!("Plus"),
            analyse_lexicale::Token::Moins => println!("Moins"),
            analyse_lexicale::Token::Mult => println!("Mult"),
            analyse_lexicale::Token::Div => println!("Div"),
        }
    }


}
