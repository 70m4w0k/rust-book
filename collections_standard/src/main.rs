mod exercices;
use exercices::*;
use std::collections::HashMap;

fn main() {
    // ----- VECTEURS -----

    // Créer un vecteur

    let v: Vec<i32> = Vec::new();

    let mut v = vec![1, 2, 3];

    // Modifier les éléments d'un vecteur

    v.push(54);
    v.push(100);
    v.push(259);

    // Lister les éléments d'un vecteur

    let troisieme: &i32 = &v[2];

    println!("3eme élément de la liste : {}", troisieme);

    match v.get(2) {
        Some(troisieme) => println!("Le 3eme élément est {}", troisieme),
        None => println!("Il n'y a pas de 3eme élément"),
    }

    // let existe_pas = &v[100]; // panic!
    let existe_pas = v.get(100); // pas de panic! => Some() & None

    // Itérer sur les vcaleurs d'un vecteur

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    // Utiliser une énum pour stocker différents types

    enum Cellule {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let ligne = vec![
        Cellule::Int(3),
        Cellule::Text(String::from("bleu")),
        Cellule::Float(10.12),
    ];

    // ----- STRINGS -----

    // Créer une String

    let mut s = String::new();

    let donnee = "contenu initial";

    let s = donnee.to_string();

    // Equivalent à :

    let mut s = String::from("contenu initial");

    // Modifier une String

    s.push_str(" foo");

    s.push('o');

    let s1 = String::from("Hello, ");
    let s2 = String::from("World !");

    let s3 = s1 + &s2; // L'opérateur '+' utilise la méthode 'fn add(self, s: &str) -> String {}'

    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");

    let s7 = format!("{}-{}-{}", s4, s5, s6);

    // Indexation des Strings

    // Une String est une surcouche de Vec<u8>

    // Les slices de chaines de caractères

    let bonjour = "Здравствуйте";

    let s = &bonjour[0..4];

    // Parcourir les chaines de caractères

    print!("\nनमस्ते.chars() :\n");
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    print!("\nनमस्ते.bytes() :\n");
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    // ----- HASHMAPS -----

    // Créer une nouvelle table de hachage

    let mut scores = HashMap::new();

    scores.insert(String::from("bleu"), 10);
    scores.insert(String::from("jaune"), 50);

    let equipes = vec![String::from("Bleu"), String::from("Jaune")];
    let scores_initiaux = vec![10, 50];

    let mut scores: HashMap<_, _> = equipes
        .into_iter()
        .zip(scores_initiaux.into_iter())
        .collect();

    // Tables de hashage et possession

    let nom_champ = String::from("Couleur favorite");
    let valeur_champ = String::from("Bleu");

    let mut table = HashMap::new();
    table.insert(nom_champ, valeur_champ);

    // nom_champ et valeur_champ ne sont plus en vigueur à partir d'ici

    // Accéder aux valeurs

    let nom_equipe = String::from("Bleu");

    let score = scores.get(&nom_equipe);

    for (cle, valeur) in &scores {
        println!("{}: {}", cle, valeur);
    }

    // Modifier les valeurs d'une table de hashage

    println!("{:?}", scores);

    scores.insert(String::from("Bleu"), 25); // Réécrire une valeur existante

    println!("{:?}", scores);

    scores.entry(String::from("Jaune")).or_insert(50); // Ajouter une valeur seulement si la clé n'a pas déjà de valeur
    scores.entry(String::from("Vert")).or_insert(75);

    println!("{:?}", scores);

    let texte = "bonjour le monde magnifique monde";

    let mut table = HashMap::new();

    for mot in texte.split_whitespace() {
        let compteur = table.entry(mot).or_insert(0);
        *compteur += 1;
    }

    println!("{:?}", table);

    // ----- EXERCICES -----

    exercices::exercices::exercice1();
}
