use std::cell::RefCell;
use std::rc::Rc;
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

pub fn run() {
    let valeur = Rc::new(RefCell::new(5));
    println!("\n--- Reference Counting ---\n");
    let a = Rc::new(Cons(Rc::clone(&valeur), Rc::new(Nil)));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *valeur.borrow_mut() += 10;

    println!("a after operations = {:?}", a);
    println!("b after operations = {:?}", b);
    println!("c after operations = {:?}", c);

    // println!("count after creating b = {}", Rc::strong_count(&a));
    // {
    //     let c = Cons(4, Rc::clone(&a));
    //     println!("count after creating c = {}", Rc::strong_count(&a));
    // }

    println!(
        "\ncount after c goes out of scope = {}",
        Rc::strong_count(&a)
    );
}
