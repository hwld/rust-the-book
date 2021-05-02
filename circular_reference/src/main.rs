use std::{cell::RefCell, clone, rc::Rc, sync::Arc, thread};

struct Node {
    name: String,
    neighbors: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(name: &str) -> Rc<Self> {
        Rc::new(Self {
            name: String::from(name),
            neighbors: RefCell::new(Vec::new()),
        })
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropped: {}", self.name);
    }
}


trait Foo {
    fn bar();
}

fn main() {
    let a = Node::new("a");
    let b = Node::new("b");
    {
        let c = Node::new("c");

        a.neighbors.borrow_mut().push(Rc::clone(&b));
        b.neighbors.borrow_mut().push(Rc::clone(&a));
        c.neighbors.borrow_mut().push(Rc::clone(&a));

        println!("a counter = {}", Rc::strong_count(&a));
        println!("b counter = {}", Rc::strong_count(&b));
        println!("c counter = {}", Rc::strong_count(&c));

        // cがスコープから抜けるとdropされる。
    }

    // cのRc<Node>のカウンタが0になり開放されたので、cが保持していたaのRc<Node>のdropも呼ばれ、カウンタが1つ減る
    println!("\na counter = {}", Rc::strong_count(&a));
    println!("b counter = {}", Rc::strong_count(&b));
    // a,bはRc<Node>のカウンタが0にならないので、保持しているb,cのRc<Node>のdropも呼ばれないため、カウンタが1にしかならず、メモリが開放されない。
}