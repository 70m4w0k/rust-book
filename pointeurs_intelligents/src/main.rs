mod boxes;
mod deref;
mod drop;
mod interior_mutability;
mod reference_counting;

fn main() {
    boxes::run();

    deref::run();

    drop::run();

    reference_counting::run();
}
