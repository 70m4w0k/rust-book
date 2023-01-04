use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Devinez le nombre !");

    let nombre_secret = rand::thread_rng().gen_range(1..101);

    println!("Le nombre secret est : {}", nombre_secret);

    loop {
        println!("Veuillez entrer un nombre");

        let mut supposition = String::new();

        io::stdin()
            .read_line(&mut supposition)
            .expect("Echec de la lecture de l'entrée utilisateur");

        let supposition: u32 = supposition
            .trim()
            .parse()
            .expect("Veuillez entrer un nombre compris entre 1 et 100");

        println!("Votre nombre est : {}", supposition);

        match supposition.cmp(&nombre_secret) {
            Ordering::Less => println!("C'est plus !"),
            Ordering::Greater => println!("C'est moins !"),
            Ordering::Equal => {
                println!("Vous avez gagné !");
                break;
            }
        }
    }
}
