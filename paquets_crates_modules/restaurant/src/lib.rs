mod salle_a_manger;

use salle_a_manger::accueil as salle_a_accueil;

pub fn manger_au_restaurant() {
    let mut repas = salle_a_manger::cuisines::PetitDejeuner::en_ete("seigle");

    repas.tartine_grillee = String::from("blé");

    println!(
        "Je voudrais une tartine grillée au {}, s'il vous plaît",
        repas.tartine_grillee
    );

    let commande_1 = salle_a_manger::cuisines::AmuseBouche::Soupe;
    let commande_2 = salle_a_manger::cuisines::AmuseBouche::Salade;

    // repas.fruit_de_saison = String::from("Myrtilles");
    // crate::salle_a_manger::accueil::ajouter_a_la_liste_attente();

    // salle_a_manger::accueil::ajouter_a_la_liste_attente();

    salle_a_accueil::ajouter_a_la_liste_attente();
    salle_a_accueil::ajouter_a_la_liste_attente();
    salle_a_accueil::ajouter_a_la_liste_attente();
}
