fn main() {
    // if let

    let couleur_favorite: Option<&str> = None;
    let on_est_mardi = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(couleur) = couleur_favorite {
        println!(
            "Utilisation de votre couleur favorite, {}, comme couleur de fond",
            couleur
        );
    } else if on_est_mardi {
        println!("Mardi, c'est le jour vert");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Utilisation du violet comme couleur de fond");
        } else {
            println!("Utilisation du orange comme couleur de fond");
        }
    } else {
        println!("Utilisation du bleu comme couleur de fond");
    }

    // while let

    let mut pile = Vec::new();

    pile.push(1);
    pile.push(2);
    pile.push(3);

    while let Some(donne_du_haut) = pile.pop() {
        println!("donnee du haut : {}", donne_du_haut);
    }

    // for loop

    let v = vec!['a', 'b', 'c', 'd'];

    for (indice, valeur) in v.iter().enumerate() {
        println!("{} est à l'indice {}", valeur, indice);
    }

    // variables masquées

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("On a 50"),
        Some(y) => println!("Correspondance, y = {:?}", y), // y dans match != y
        _ => println!("Cas par défaut, x = {:?}", x),
    }
    println!("A la fin, x = {:?}, y = {:?}", x, y);

    // Plusieurs motifs

    let x = 5;

    match x {
        1 | 2 => println!("1 ou 2"),
        3..=5 => println!("de 3 à 5"),
        _ => println!("Quelque chose d'autre"),
    }

    let x = 'k';

    match x {
        'a'..='j' => println!("lettre ASCII du début"),
        'k'..='z' => println!("lettre ASCII de la fin"),
        _ => println!("Autre chose"),
    }

    // Destructurer

    let p = Point { x: 1, y: 7 };

    let Point { x, y } = p;

    assert_eq!(1, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("Sur l'axe x à la position {}", x),
        Point { x: 0, y } => println!("Sur l'axe y à la position {}", y),
        Point { x, y } => println!("Sur aucun des axes ({}, {})", x, y),
    }

    // Destructurer une enum

    // let msg = Message::Quitter;
    // let msg = Message::Deplacer { x: 50, y: 100 };
    // let msg = Message::Ecrire(String::from("Hello, World !"));
    let msg = Message::ChangerCouleur(Couleur::Tsv(0, 160, 255));

    match msg {
        Message::Quitter => {
            println!("la variante Quitter n'a pas de donnée à déstructurer");
        }
        Message::Deplacer { x, y } => {
            println!("Déplacement de {} sur l'axe x et de {} sur l'axe y", x, y);
        }
        Message::Ecrire(text) => println!("Message textuel : {}", text),
        Message::ChangerCouleur(Couleur::Rvb(r, v, b)) => println!(
            "Changement des taux de rouge à {}, de vert à {} et de bleu à {}",
            r, v, b
        ),
        Message::ChangerCouleur(Couleur::Tsv(t, s, v)) => println!(
            "Changement des taux de teinte à {}, de saturation à {} et de valeur à {}",
            t, s, v
        ),
        _ => (),
    }

    // Destructurer structures et tuples

    let ((pieds, pouces), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // Ignorer les parties d'une valeur avec le _ imbriqué

    let mut valeur_du_reglage = None;
    let nouvelle_valeur_du_reglage = Some(10);

    match (valeur_du_reglage, nouvelle_valeur_du_reglage) {
        (Some(_), Some(_)) => {
            println!("Vous ne pouvez pas écraser la valeur déjà existante");
        }
        _ => {
            valeur_du_reglage = nouvelle_valeur_du_reglage;
        }
    }

    println!("Le réglage vaut : {:?}", valeur_du_reglage);

    // Ignorer des éléments avec ..

    let nombres = (2, 4, 8, 16, 32);

    match nombres {
        (premier, .., dernier) => {
            println!(
                "Voici le premier et dernier nombre : {}, {}",
                premier, dernier
            );
        }
    }

    // Les contrôles de correspondance

    let nombre = Some(3);

    match nombre {
        Some(x) if x % 2 == 0 => println!("Le nombre {} est pair", x),
        Some(x) => println!("Le nombre {} est impair", x),
        None => (),
    }

    // Capturer les valeurs avec @

    let msg = Message2::Hello { id: 5 };

    match msg {
        Message2::Hello {
            id: id_variable @ 3..=7,
        } => println!(
            "Nous avons trouvé un id dans l'intervalle : {}",
            id_variable
        ),
        Message2::Hello {
            id: id_variable @ 10..=12,
        } => println!("Nous avons trouvé un id dans un autre intervalle "),
        Message2::Hello { id } => println!("Nous avons trouvé un autre id : {}", id),
    }
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quitter,
    Deplacer { x: i32, y: i32 },
    Ecrire(String),
    ChangerCouleur(Couleur),
}

enum Couleur {
    Rvb(i32, i32, i32),
    Tsv(i32, i32, i32),
}

enum Message2 {
    Hello { id: i32 },
}
