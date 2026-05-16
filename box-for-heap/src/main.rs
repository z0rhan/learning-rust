use List::{Cons, Nil};
use std::fmt::Debug;
use std::ops::Deref;
use std::ops::DerefMut;
use std::mem;

// From &T to &U when T: Deref<Target = U>
// From &mut T to &mut U when T: DerefMut<Target = U>
// From &mut T to &U when T: Deref<Target = U>
// immutable ref cannot be coerced to mutable ref

fn main() {
    box_for_heap();

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(x , 5);
    // Here, we can deref y because we implemented the Deref trait for MyBox
    // BTS *y = *(y.deref())
    assert_eq!(*y, 5);

    // Deref coercion
    // So basically conversion of ref to a type that implements Deref to
    // ref of another type
    hello("Rust");
    // Deref coercion makes it possible to make the function_call below
    // So, str is coerced to &str by str.deref() -> &String and .deref() again
    // to get &str
    let str = Box::new("Rust");
    hello(&str);
    // Without deref coercion, it would be,
    hello(&(*str)[..]);

    // Likewise for mutable dereferencing, we have DerefMut trait
    let mut z = MyBox::new(5);
    println!("z : {}", *z);
    *z = 6;
    println!("z : {}", *z);

    // Sometime we want to free the resources before the end of scope
    // In that case, we cannot call the drop method ourselves
    // Instead we can use the std::mem::drop function provided by std lib
    mem::drop(y);
    // z(6) is dropped first before y(5) is dropped at the end of the scope
    // but we changed the order by calling std::mem::drop(y);
}

fn hello(str: &str) {
    println!("Hello, {str}")
}

fn box_for_heap() {
    let x = 5;
    // y is now a ref to x
    let y = &x;
    // Similary, z is now a ref but to a copied value of x on the heap
    let z = Box::new(x);

    assert_eq!(x, 5);
    // Here, we have to deref y to get to value of x (5)
    assert_eq!(*y, 5);
    // Here, we have to deref z to get to its value of 5 on the heap
    assert_eq!(*z, 5);

    // Box allows to store data on the heap rather than on stack
    // We can use them like any other variables on the stack
    let b = Box::new(8);
    println!("b = {b}");

    // Helps with defining recursive data structure such as cons below
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

enum List {
    // With just List, the size is not known at compile time
    // Box helps with that
    Cons(i32, Box<List>),
    Nil
}

// Tuple struct
struct MyBox<T>(T)
where
    T: Debug;

impl<T> MyBox<T>
where
    T: Debug
{
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>
where
    T: Debug
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T>
where
    T: Debug
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// Drop trait something like destructor in cpp
impl<T> Drop for MyBox<T>
where
    T: Debug
{
    fn drop(&mut self) {
        println!("Dropping MyBox with data: {:?}", self.0)
    }
}
