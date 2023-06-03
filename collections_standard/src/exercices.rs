pub mod exercices {
    use std::{collections::HashMap, vec};

    // À partir d'une liste d'entiers, utiliser un vecteur et retourner la médiane
    // (la valeur au milieu lorsque la liste est triée) et le mode
    // (la valeur qui apparaît le plus souvent ;
    // une table de hachage sera utile dans ce cas) de la liste.

    pub fn exercice1() {
        println!("exercice 1");

        let mut vec1 = vec![10, 9, 6, 4, 5, 15, 23, 89, 6, 54, 52, 5, 152, 457];
        let mut vec2 = vec![
            11, 9, 6, 4, 5, 15, 10, 6, 96, 34, 54, 25, 65, 7, 85, 96, 3, 2, 11, 4, 5, 152, 457,
        ];

        let mut mediane1 = mediane(&mut vec1);
        let mediane2 = mediane(&mut vec2);

        println!("mediane du vecteur {:?} : {}", vec1, mediane1);
        println!("mediane du vecteur {:?} : {}", vec2, mediane2);

        let m = mode(&mut vec1);
        let n = mode(&mut vec2);
    }

    fn mediane(v: &mut Vec<i32>) -> i32 {
        let mut mediane = 0;
        v.sort();

        match v.get(v.len() / 2) {
            Some(x) => mediane = *x,
            None => (),
        };

        mediane
    }

    fn mode(v: &mut Vec<i32>) -> i32 {
        v.sort();
        let mut mode = HashMap::new();

        let mut max = 0;

        for i in v {
            let compteur = mode.entry(i).or_insert(0);
            *compteur += 1;
            if i > &mut max {
                max = *i;
            }
        }

        println!("mode : {:?}", mode);
        println!("max : {:?}", max);

        0
    }

    pub fn exercice2() {}
}
