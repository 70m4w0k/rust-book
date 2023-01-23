// ----- DEFINIR UNE ENUM -----

enum SorteAdresseIP {
    V4,
    V6,
}

enum AdresseIP {
    V4(String),
    V6(String),
}

enum Message {
    Quitter,
    Deplacer { x: i32, y: i32 },
    Ecrire(String),
    ChangerCouleur(i32, i32, i32),
}

// Equivalent à ...

struct MessageQuitter;
struct MessageDeplacer {
    x: i32,
    y: i32,
}
struct MessageEcrire(String);
struct MessageChangerCouleur(i32, i32, i32);

impl Message {
    fn appeler(&self) {}
}

// ----- MATCH -----

enum PieceUs {
    Penny,
    Nickel,
    Dime,
    Quarter(EtatUS),
}

#[derive(Debug)]
enum EtatUS {
    Alabama,
    Alaska,
}

fn main() {
    // ----- DEFINIR UNE ENUM -----

    let quatre = SorteAdresseIP::V4;
    let six = SorteAdresseIP::V6;
    router(quatre);
    router(six);

    let local = AdresseIP::V4(String::from("127.0.0.1"));
    let rebouclage = AdresseIP::V6(String::from("::1"));

    let message = Message::Ecrire(String::from("hello"));
    message.appeler();

    let cinq = Some(5);
    let six = plus_un(cinq);
    let none = plus_un(None);

    // ----- MATCH -----

    let jete_de_de = 9;

    match jete_de_de {
        3 => ajouter_chapeau_fantaisie(),
        7 => enlever_chapeau_fantaisie(),
        _ => (), // _ : Motif Générique
    }

    // ----- IF LET -----

    let une_valeur_u8 = Some(3u8);
    match une_valeur_u8 {
        Some(max) => println!("le maximum est réglé sur {}", max),
        _ => (),
    }

    // Equivalent à ...

    let une_valeur_u8 = Some(3u8);
    if let Some(max) = une_valeur_u8 {
        println!("le maximum est réglé sur {}", max);
    }

    // pouvez considérer le if let comme du sucre syntaxique
    // pour un match qui exécute du code uniquement
    // quand la valeur correspond à un motif donné
    // et ignore toutes les autres valeurs

    let mut piece = PieceUs::Quarter(EtatUS::Alabama);

    // let mut compteur = 0;
    // match piece {
    //     PieceUs::Quarter(etat) => println!("Il s'agit d'un quarter de l'Etat de {:?} ! ", etat),
    //     _ => compteur += 1,
    // }

    // Equivalent à ...

    let mut compteur = 0;
    if let PieceUs::Quarter(etat) = piece {
        println!("Il s'agit d'un quarter de l'Etat de {:?} ! ", etat);
    } else {
        compteur += 1;
    }
}

fn router(adresse_ip: SorteAdresseIP) {}

// ----- MATCH -----

fn valeur_en_centimes(piece: PieceUs) -> u8 {
    match piece {
        PieceUs::Penny => {
            println!("Un centime porte bonheur !");
            1
        }
        PieceUs::Nickel => 5,
        PieceUs::Dime => 10,
        PieceUs::Quarter(etat) => {
            println!("Il s'agit d'un quarter de l'Etat de {:?} ! ", etat);
            25
        }
    }
}

fn plus_un(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn ajouter_chapeau_fantaisie() {}
fn enlever_chapeau_fantaisie() {}
