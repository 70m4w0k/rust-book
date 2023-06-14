struct Compteur {
    compte: u32,
}

impl Compteur {
    fn new() -> Compteur {
        Compteur { compte: 0 }
    }
}

impl Iterator for Compteur {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.compte < 5 {
            self.compte += 1;
            Some(self.compte)
        } else {
            None
        }
    }
}

#[test]
fn appeler_next_directement() {
    let mut compteur = Compteur::new();

    assert_eq!(compteur.next(), Some(1));
    assert_eq!(compteur.next(), Some(2));
    assert_eq!(compteur.next(), Some(3));
    assert_eq!(compteur.next(), Some(4));
    assert_eq!(compteur.next(), Some(5));
    assert_eq!(compteur.next(), None);
}

#[test]
fn autre_methode() {
    let sum: u32 = Compteur::new()
        .zip(Compteur::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    assert_eq!(sum, 18);
}

pub fn run() {}
