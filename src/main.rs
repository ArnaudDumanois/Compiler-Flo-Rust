extern crate core;

mod analyse_lexicale;
mod analyse_syntaxique;
mod arbre_abstrait;
mod table_symbole;
mod generation_code;

fn main() {

    let input = "Vrai + 3;";
    let tokens = analyse_lexicale::lexer(input);
    println!("Tokens: {:?}", tokens);


}
