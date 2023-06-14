#[derive(PartialEq, Debug)]
struct Chaussure {
    pointure: u32,
    style: String,
}

fn chaussures_a_ma_taille(chaussures: Vec<Chaussure>, pointure: u32) -> Vec<Chaussure> {
    chaussures
        .into_iter()
        .filter(|c| c.pointure == pointure)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_by_size() {
        let chaussures = vec![
            Chaussure {
                pointure: 43,
                style: String::from("sneaker"),
            },
            Chaussure {
                pointure: 44,
                style: String::from("baskets"),
            },
            Chaussure {
                pointure: 44,
                style: String::from("sandales"),
            },
            Chaussure {
                pointure: 45,
                style: String::from("bottes"),
            },
            Chaussure {
                pointure: 43,
                style: String::from("chaussons"),
            },
        ];
        let a_ma_taille = chaussures_a_ma_taille(chaussures, 44);

        assert_eq!(
            a_ma_taille,
            vec![
                Chaussure {
                    pointure: 44,
                    style: String::from("baskets"),
                },
                Chaussure {
                    pointure: 44,
                    style: String::from("sandales"),
                }
            ]
        );
    }
}
