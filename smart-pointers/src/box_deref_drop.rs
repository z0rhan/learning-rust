use std::fmt::Debug;
use std::ops::Deref;
use std::ops::DerefMut;

pub enum List {
    // With just List, the size is not known at compile time
    // Box helps with that
    Cons(i32, Box<List>),
    Nil
}

// Tuple struct
pub struct MyBox<T>(T)
where
    T: Debug;

impl<T> MyBox<T>
where
    T: Debug
{
    pub fn new(x: T) -> MyBox<T> {
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
