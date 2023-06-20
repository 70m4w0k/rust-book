/*
pub struct Billet {
    etat: Option<Box<dyn Etat>>,
    contenu: String,
}

impl Billet {
    pub fn new() -> Billet {
        Billet {
            etat: Some(Box::new(Brouillon {})),
            contenu: String::new(),
        }
    }

    pub fn ajouter_texte(&mut self, texte: &str) {
        self.contenu.push_str(texte);
    }

    pub fn contenu(&self) -> &str {
        self.etat.as_ref().unwrap().contenu(self)
    }

    pub fn demander_relecture(&mut self) {
        if let Some(s) = self.etat.take() {
            self.etat = Some(s.demander_relecture())
        }
    }

    pub fn approuver(&mut self) {
        if let Some(s) = self.etat.take() {
            self.etat = Some(s.approuver())
        }
    }
}

trait Etat {
    fn demander_relecture(self: Box<Self>) -> Box<dyn Etat>;
    fn approuver(self: Box<Self>) -> Box<dyn Etat>;
    fn contenu<'a>(&self, billet: &'a Billet) -> &'a str {
        ""
    }
}

struct Brouillon {}

impl Etat for Brouillon {
    fn demander_relecture(self: Box<Self>) -> Box<dyn Etat> {
        Box::new(EnRelecture {})
    }

    fn approuver(self: Box<Self>) -> Box<dyn Etat> {
        self
    }
}

struct EnRelecture {}

impl Etat for EnRelecture {
    fn demander_relecture(self: Box<Self>) -> Box<dyn Etat> {
        self
    }

    fn approuver(self: Box<Self>) -> Box<dyn Etat> {
        Box::new(Publier {})
    }
}

struct Publier {}

impl Etat for Publier {
    fn demander_relecture(self: Box<Self>) -> Box<dyn Etat> {
        self
    }

    fn approuver(self: Box<Self>) -> Box<dyn Etat> {
        self
    }

    fn contenu<'a>(&self, billet: &'a Billet) -> &'a str {
        &billet.contenu
    }
}
*/

// Implémentation avec des types

pub struct Billet {
    contenu: String,
}

pub struct BrouillonDeBillet {
    contenu: String,
}

impl Billet {
    pub fn new() -> BrouillonDeBillet {
        BrouillonDeBillet {
            contenu: String::new(),
        }
    }

    pub fn contenu(&self) -> &str {
        &self.contenu
    }
}

impl BrouillonDeBillet {
    pub fn ajouter_texte(&mut self, texte: &str) {
        self.contenu.push_str(texte);
    }

    pub fn demander_relecture(self) -> BilletEnRelecture {
        BilletEnRelecture {
            contenu: self.contenu,
        }
    }
}

pub struct BilletEnRelecture {
    contenu: String,
}

impl BilletEnRelecture {
    pub fn approuver(self) -> Billet {
        Billet {
            contenu: self.contenu,
        }
    }
}
