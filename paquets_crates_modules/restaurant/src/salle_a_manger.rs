
pub mod accueil {
    pub fn ajouter_a_la_liste_attente() {}

    fn installer_a_une_table() {}
}

mod service {
    fn prendre_une_commande() {}

    fn servir_commande() {}

    fn encaisser() {}
}

pub mod cuisines {
    pub enum AmuseBouche {
        Soupe,
        Salade,
    }

    pub struct PetitDejeuner {
        pub tartine_grillee: String,
        fruit_de_saison: String,
    }

    impl PetitDejeuner {
        pub fn en_ete(tartine_grillee: &str) -> PetitDejeuner {
            PetitDejeuner {
                tartine_grillee: String::from(tartine_grillee),
                fruit_de_saison: String::from("pêches"),
            }
        }
    }

    fn corriger_commande_erronnee() {
        cuisiner_commande();
        // super::servir_commande();
    }

    fn cuisiner_commande() {}
}
