use generiques::*;

fn main() {
    // intro();
    // types_de_donnees_generiques();
    // les_traits();
    les_durees_de_vie();
}

fn intro() {
    let liste_de_nombres = vec![34, 50, 25, 100, 65];

    let resultat = le_plus_grand_i32(&liste_de_nombres);
    println!("Le plus grand nombre est {}", resultat);

    let resultat = le_plus_grand(&liste_de_nombres);
    println!("Le plus grand nombre est {}", resultat);

    let liste_de_caracteres = vec!['y', 'm', 'a', 'q'];

    let resultat = le_plus_grand_caractere(&liste_de_caracteres);
    println!("Le plus grand caractère est {}", resultat);

    let resultat = le_plus_grand(&liste_de_caracteres);
    println!("Le plus grand caractère est {}", resultat);
}

fn le_plus_grand_i32(liste: &[i32]) -> i32 {
    let mut le_plus_grand = liste[0];

    for &element in liste {
        if element > le_plus_grand {
            le_plus_grand = element;
        }
    }
    le_plus_grand
}

fn le_plus_grand_caractere(liste: &[char]) -> char {
    let mut le_plus_grand = liste[0];

    for &element in liste.iter() {
        if element > le_plus_grand {
            le_plus_grand = element;
        }
    }

    le_plus_grand
}

fn le_plus_grand<T: std::cmp::PartialOrd + Copy>(liste: &[T]) -> T {
    let mut le_plus_grand = liste[0];

    for &element in liste.iter() {
        if element > le_plus_grand {
            le_plus_grand = element;
        }
    }

    le_plus_grand
}
