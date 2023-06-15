use cargo_crates::mixer;
use cargo_crates::CouleurPrimaire;

fn main() {
    let rouge = CouleurPrimaire::Rouge;
    let jaune = CouleurPrimaire::Jaune;
    mixer(rouge, jaune);
}
