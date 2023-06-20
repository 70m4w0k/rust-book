use gui::{Affichable, Bouton, Ecran};

struct ListeDeroulante {
    largeur: u32,
    hauteur: u32,
    options: Vec<String>,
}

impl Affichable for ListeDeroulante {
    fn afficher(&self) {}
}

fn main() {
    let ecran = Ecran {
        composants: vec![
            Box::new(ListeDeroulante {
                largeur: 75,
                hauteur: 10,
                options: vec![
                    String::from("Oui"),
                    String::from("Peur-etre"),
                    String::from("Non"),
                ],
            }),
            Box::new(Bouton {
                largeur: 50,
                hauteur: 10,
                libelle: String::from("OK");
            }),
        ],
    };

    ecran.executer();
}
