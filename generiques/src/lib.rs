use std::fmt::{Debug, Display};

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

pub fn types_de_donnees_generiques() {
    let entiers = Point { x: 5, y: 10 };
    let flottants = Point { x: 5.3, y: 10.0 };
    let un_entier_et_un_flottant = MultiPoint { x: 5, y: 4.5 };

    println!("entiers.x = {}", entiers.x());

    let p1 = MultiPoint {
        x: "Hello",
        y: "World",
    };

    let p2 = p1.melange(un_entier_et_un_flottant);

    println!("p2.x = {}, p2.y = {}", p2.x, p2.y);
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_depuis_lorigine(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct MultiPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> MultiPoint<T, U> {
    fn melange<V, W>(self, other: MultiPoint<V, W>) -> MultiPoint<T, W> {
        MultiPoint {
            x: self.x,
            y: other.y,
        }
    }
}

/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

pub fn les_traits() {
    let tweet = Tweet {
        nom_utilisateur: String::from("jean"),
        contenu: String::from("Bien sur bien sur"),
        reponse: false,
        retweet: false,
    };

    println!("1 nouveau tweet : {}", tweet.resumer());

    notifier(&tweet);

    println!("{}", retourne_resumable(true).resumer());
    println!("{}", retourne_resumable(false).resumer());
}

pub trait Resumable {
    fn resumer_auteur(&self) -> String;
    fn resumer(&self) -> String {
        format!("(Lire plus d'éléments de {} ...)", self.resumer_auteur())
    }
}

pub struct ArticleDePresse {
    pub titre: String,
    pub lieu: String,
    pub auteur: String,
    pub contenu: String,
}

impl Resumable for ArticleDePresse {
    fn resumer_auteur(&self) -> String {
        format!("{}", self.auteur)
    }

    fn resumer(&self) -> String {
        format!("{}, par {}({})", self.titre, self.auteur, self.contenu)
    }
}

pub struct Tweet {
    pub nom_utilisateur: String,
    pub contenu: String,
    pub reponse: bool,
    pub retweet: bool,
}

impl Resumable for Tweet {
    fn resumer_auteur(&self) -> String {
        format!("@{}", self.nom_utilisateur)
    }

    fn resumer(&self) -> String {
        format!("{} : {}", self.nom_utilisateur, self.contenu)
    }
}

pub fn notifier(element: &impl Resumable) {
    println!("Flash info ! {}", element.resumer());
}

pub fn notifier_2<T: Resumable + Display>(element: &T) {
    println!("Flash info ! {:?}", element.resumer());
}

pub fn une_fonction<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

pub fn retourne_resumable(estArticle: bool) -> Box<dyn Resumable> {
    if estArticle {
        Box::new(ArticleDePresse {
            titre: String::from("Les Penguins ont remporté la Coupe Stanley !"),
            lieu: String::from("Pittsburgh, PA, USA"),
            auteur: String::from("Iceburgh"),
            contenu: String::from(
                "Les Penguins de Pittsburgh sont une nouvelle fois la \
            meilleure équipe de hockey de la LNH.",
            ),
        })
    } else {
        Box::new(Tweet {
            nom_utilisateur: String::from("jean"),
            contenu: String::from("Bien sûr, les amis, comme vous le savez probablement déjà"),
            reponse: false,
            retweet: false,
        })
    }
}

/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

pub fn les_durees_de_vie() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let resultat = la_plus_longue(string1.as_str(), string2);
    println!("La plus longue chaîne est {}", resultat);
    println!(
        "La plus longue chaîne est {}",
        la_plus_longue_avec_annonce(string1.as_str(), string2, "attention")
    );
}

fn la_plus_longue<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn la_plus_longue_avec_annonce<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Annonce ! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
