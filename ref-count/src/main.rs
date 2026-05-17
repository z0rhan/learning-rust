use std::{cell::RefCell, rc::{Rc, Weak}};

use ref_count::refcount::ImmutList::{ImmutCons, ImmutNil};
use ref_count::refcount::MutList::{MutCons, MutNil};
use ref_count::refcount::List::{Cons, Nil};
use ref_count::refcount::Node;

fn main() {
    let shared = Rc::new(ImmutCons(5, Rc::new(ImmutCons(10, Rc::new(ImmutNil)))));
    let first = ImmutCons(3, Rc::clone(&shared));
    let second = ImmutCons(4, Rc::clone(&shared));
    println!("Ref count of shared: {}", Rc::strong_count(&shared));
    {
        let _third= ImmutCons(2, Rc::clone(&shared));
        println!("Ref count of shared: {}", Rc::strong_count(&shared));
    }
    println!("Ref count of shared: {}", Rc::strong_count(&shared));

    println!("shared: {:?}", shared);
    println!("fist: {:?}", first);
    println!("second: {:?}", second);

    // Rc only allows to have immutable references
    // Drop for Rc<T> decreases the ref count
    // Finally when ref count is zeor, the resource is cleaned up
    // Also, Rc is only for use in single-threaded scenarios

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(MutCons(Rc::clone(&value), Rc::new(MutNil)));
    let b = MutCons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = MutCons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    println!("Before: {:?}", a);
    println!("Before: {:?}", b);
    println!("Before: {:?}", c);
    *value.borrow_mut() += 2;
    println!("After: {:?}", a);
    println!("After: {:?}", b);
    println!("After: {:?}", c);

    let i = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    let ii = Rc::new(Cons(10, RefCell::new(Rc::clone(&i))));
    
    if let Some(tail) = i.tail() {
        *tail.borrow_mut() = Rc::clone(&ii);
    }

    // Here there is a cycle of referenc which causes memory leak at the end of
    // scope
    println!("Ref count of i: {}", Rc::strong_count(&i));
    println!("Ref count of i: {}", Rc::strong_count(&ii));

    let leaf = Rc::new(Node::new(
            5,
            RefCell::new(Weak::new()),
            RefCell::new(vec![])
        ));
    let branch = Rc::new(Node::new(
            9,
            RefCell::new(Weak::new()),
            RefCell::new(vec![Rc::clone(&leaf)])
        ));

    println!("Leaf parent: {:?}", leaf.parent().borrow().upgrade());
    *leaf.parent().borrow_mut() = Rc::downgrade(&branch);
    println!("Leaf parent: {:?}", leaf.parent().borrow().upgrade());
}
