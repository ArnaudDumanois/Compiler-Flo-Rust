mod analyse_lexicale;
mod analyse_syntaxique;
mod arbre_abstrait;
mod table_symbole;
mod generation_code;

fn main() {

    let input = "1 + 1";
    let tokens = analyse_lexicale::lexer(input);
    println!("Tokens: {:?}", tokens);


}
