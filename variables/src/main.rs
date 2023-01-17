fn main() {
    let mut x = 5;
    println!("La valeur de x est : {}", x);
    x = 6;
    println!("La valeur de x est : {}", x);

    // Constantes
    const TROIS_HEURES_EN_SECONDES: u32 = 60 * 60 * 3;
    println!("3 Heures en secondes : {}", TROIS_HEURES_EN_SECONDES);

    // Masquage
    {
        let x = x * 2;
        println!("La valeur de x dans la portée interne est : {}", x);
    }
    println!("La valeur de x est : {}", x);

    // Les types de données

    let _x = 2.0;

    let y: f32 = 3.0;

    // Opérations

    // addition
    let _somme = 5 + 10;

    // soustraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _produit = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _arrondi = 2 / 3; // retournera 0

    // modulo
    let _reste = 43 % 5;

    // Booleen

    let _t = true;

    let _f: bool = false; // avec une annotation de type explicite

    // Caractère
    let _c = 'z';
    let _z = 'ℤ';
    let _chat_aux_yeux_de_coeur = '😻';

    // Tuples

    let tup = (500, 6.4, 1);

    let (_x, _y, _z) = tup;

    println!("La valeur de y est : {}", y); // 6.4

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let _cinq_cents = x.0;

    let _six_virgule_quatre = x.1;

    let _un = x.2;

    // Tableau
    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    let _a = [3; 5]; // let a = [3, 3, 3, 3, 3];

    let a = [1, 2, 3, 4, 5];

    let _premier = a[0];
    let _second = a[1];

    // FONCTIONS
    une_autre_fonction(5);

    afficher_mesure_avec_unite(5, 'h');

    // Instructions

    let y = {
        let x = 3;
        x + 1
    };

    println!("La valeur de y est : {}", y);

    // Fonctions qui retournent des valeurs

    let x = plus_un(5);

    println!("La valeur de x est : {}", x);

    // STRUCTURES DE CONTROLE

    // if

    let nombre = 3;

    if nombre < 5 {
        println!("La condition est vérifiée");
    } else {
        println!("La condition n'est pas vérifiée");
    }

    let condition = true;
    let nombre = if condition { 5 } else { 6 };

    println!("La valeur du nombre est : {}", nombre);

    // Boucle loop

    // loop {
    //     println!("À nouveau !");
    // }

    let mut compteur = 0;

    // L'étiquette 'increment spécifie à quelle boucle doivent se référer les instructions break

    'increment: loop {
        println!("compteur = {}", compteur);
        let mut restant = 10;

        loop {
            println!("restant = {}", restant);
            if restant == 9 {
                break;
            }
            if compteur == 2 {
                break 'increment;
            }
            restant -= 1;
        }

        compteur += 1;
    }
    println!("Fin du compteur = {}", compteur);

    // Boucle while

    let mut nombre = 3;

    while nombre != 0 {
        println!("{} !", nombre);

        nombre -= 1;
    }

    println!("DÉCOLLAGE !!!");

    // Boucle for

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("La valeur est : {}", element);
    }
}

fn une_autre_fonction(x: i32) {
    println!("La valeur de x est : {}", x);
}

fn afficher_mesure_avec_unite(valeur: i32, unite: char) {
    println!("La mesure est : {}{}", valeur, unite);
}

fn plus_un(x: i32) -> i32 {
    x + 1
}
