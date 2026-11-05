use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node {
    name: String,
    children: Vec<Rc<RefCell<Node>>>,
    back_ref: Option<Weak<RefCell<Node>>>,
}

impl Node {
    fn new(name: &str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            name: name.to_string(),
            children: Vec::new(),
            back_ref: None,
        }))
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}

fn main() {
    let a = Node::new("A");
    let b = Node::new("B");
    let c = Node::new("C");

    // A -> B -> C -> A
    a.borrow_mut().children.push(Rc::clone(&b));
    b.borrow_mut().children.push(Rc::clone(&c));
    c.borrow_mut().back_ref = Some(Rc::downgrade(&a));

    println!("A strong count: {}", Rc::strong_count(&a)); // 1 (only `a` binding)
    println!("B strong count: {}", Rc::strong_count(&b)); // 2 (b + A's child)
    println!("C strong count: {}", Rc::strong_count(&c)); // 2 (c + B's child)

    let c_ref = c.borrow();
    if let Some(back) = &c_ref.back_ref
        && let Some(a_ref) = back.upgrade()
    {
        println!("C points back to : {}", a_ref.borrow().name);
    }
}
