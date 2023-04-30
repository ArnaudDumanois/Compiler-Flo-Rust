/*
Analyseur lexical du langage Flo
 */
use std::str::CharIndices;
use std::iter::Peekable;

// on va declarer tout les tokens de notre langage ici
#[derive(Debug, PartialEq)]
pub enum Token {
    // Types
    entier,
    booleen,

    // Operateurs
    Plus,
    Moins,
    Multiplier,
    Diviser,
    Modulo,
    Egal,
    Different,
    Superieur,
    SuperieurEgal,
    Inferieur,
    InferieurEgal,
    Et,
    Ou,
    Non,

    // Mots clés
    Ecrire,
    Lire,
    Retourner,
    Si,
    Sinon,
    SinonSi,
    Tantque,
    Vrai,
    Faux,

    // Ponctuation
    ParentheseOuvrante,
    ParentheseFermante,
    AccoladeOuvrante,
    AccoladeFermante,
    PointVirgule,
    Virgule,

    // Identifiant et valeur
    Identifiant(String),
    ValeurEntier(i32),
    ValeurBooleen(bool),
    EndOfFile,
}

