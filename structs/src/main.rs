struct Utilisateur {
    actif: bool,
    pseudo: String,
    email: String,
    nombre_de_connexions: u32,
}

struct Couleur(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    largeur: u32,
    hauteur: u32,
}

impl Rectangle {
    fn aire(&self) -> u32 {
        self.largeur * self.hauteur
    }

    fn largeur(&self) -> bool {
        self.largeur > 0
    }

    fn peut_contenir(&self, autre: &Rectangle) -> bool {
        self.largeur >= autre.largeur && self.hauteur >= autre.hauteur
    }

    fn carre(cote: u32) -> Rectangle {
        Rectangle {
            largeur: cote,
            hauteur: cote,
        }
    }
}

fn main() {
    let utilisateur1 = Utilisateur {
        actif: true,
        pseudo: String::from("exemple"),
        email: String::from("exemple@mail.com"),
        nombre_de_connexions: 1,
    };

    let utilisateur2 = Utilisateur {
        email: String::from("exemple22@mail.com"),
        ..utilisateur1
    };
    println!("email: {}", utilisateur1.email);

    let rect1 = Rectangle {
        largeur: 30,
        hauteur: 50,
    };

    println!(
        "L'aire du rectangle est de {} pixels carrés (via fonction)",
        aire(&rect1)
    );
    println!(
        "L'aire du rectangle est de {} pixels carrés (via méthode)",
        rect1.aire()
    );

    println!("rect1 est {:?}", rect1);

    let echelle = 2;
    let rect1 = Rectangle {
        largeur: dbg!(30 * echelle),
        hauteur: 50,
    };

    dbg!(&rect1);

    // println!("utilisateur 1 : {}", utilisateur1);
}

fn creer_utilisateur(pseudo: String, email: String) -> Utilisateur {
    Utilisateur {
        actif: true,
        pseudo,
        email,
        nombre_de_connexions: 1,
    }
}

// fn aire(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

fn aire(rectangle: &Rectangle) -> u32 {
    rectangle.hauteur * rectangle.largeur
}
