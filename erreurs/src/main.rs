use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    erreurs_recuperables();

    let _ = gerer_les_erreurs();
    let _ = gerer_les_erreurs_2();
    let _ = gerer_les_erreurs_3();
    let _ = gerer_les_erreurs_4();

    let _ = propager_les_erreurs();
    let _ = propager_les_erreurs_2();
    let _ = propager_les_erreurs_3();
    let _ = propager_les_erreurs_4();

    let _ = dernier_caractere_de_la_premiere_ligne(
        "texte ceci est un texte, \nbonjour, ceci est un texte",
    );
}

fn erreurs_recuperables() {
    // Des erreurs récupérables avec Result<T, E>

    let f = File::open("hello.txt");

    let _f = match f {
        Ok(fichier) => fichier,

        Err(erreur) => panic!("Erreur d'ouverture du fichier : {}", erreur),
    };
}

fn gerer_les_erreurs() {
    // Gérer les différentes erreurs

    let f = File::open("hello.txt");

    let _f = match f {
        Ok(fichier) => fichier,
        Err(erreur) => match erreur.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Erreur de création du fichier : {}", e),
            },
            autre_erreur => {
                panic!("Erreur d'ouverture du fichier : {}", autre_erreur)
            }
        },
    };
}

fn gerer_les_erreurs_2() {
    // Avec les fermetures et unwrap_or_else()

    let _f = File::open("hello.txt").unwrap_or_else(|erreur| {
        if erreur.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|erreur| {
                panic!("Erreur de création du fichier : {:?}", erreur);
            })
        } else {
            panic!("Erreur d'ouverture du fichier : {:?}", erreur);
        }
    });
}

fn gerer_les_erreurs_3() {
    // Avec unwrap, une erreur par défaut

    let _f = File::open("hello.txt").unwrap();
}

fn gerer_les_erreurs_4() {
    // Avec expect, une erreur personalisée

    let _f = File::open("hello.txt").expect("Echec à l'ouverture du fichier");
}

fn propager_les_erreurs() -> Result<String, io::Error> {
    // Retourne le resultat ou l'erreur

    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(fichier) => fichier,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn propager_les_erreurs_2() -> Result<String, io::Error> {
    // Un raccourci pour propager les erreurs : `?`
    // Le code ci dessous a les mêmes fonctionnalités qu'au dessus

    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn propager_les_erreurs_3() -> Result<String, io::Error> {
    // Le code ci dessous a les mêmes fonctionnalités qu'au dessus

    let mut s = String::new();

    let _f = File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn propager_les_erreurs_4() -> Result<String, io::Error> {
    // Le code ci dessous a les mêmes fonctionnalités qu'au dessus

    fs::read_to_string("hello.txt")
}

fn dernier_caractere_de_la_premiere_ligne(texte: &str) -> Option<char> {
    texte.lines().next()?.chars().last()
}
