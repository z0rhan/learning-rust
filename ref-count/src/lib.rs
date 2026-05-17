pub mod refcount {
    use std::rc::{Rc, Weak};
    use std::cell::RefCell;

    #[derive(Debug)]
    pub enum ImmutList {
        ImmutCons(i32, Rc<ImmutList>),
        ImmutNil
    }

    #[derive(Debug)]
    pub enum MutList {
        MutCons(Rc<RefCell<i32>>, Rc<MutList>),
        MutNil
    }

    pub enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil
    }

    impl List {
        pub fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Self::Cons(_, item) => Some(item),
                Self::Nil => None
            }
        }
    }

    #[derive(Debug)]
    pub struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>
            
    }

    impl Node {
        pub fn new(
            value: i32,
            parent: RefCell<Weak<Node>>,
            children: RefCell<Vec<Rc<Node>>>
        ) -> Node
        {
            Node { value, parent, children }
        }

        pub fn parent(&self) -> &RefCell<Weak<Node>> {
            &self.parent
        }
    }
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let usage = self.value as f64 / self.max as f64;

        if usage >= 1.0 {
            self.messenger.send("Error: you are over your quota");
        }
        else if usage >= 0.9 {
            self.messenger
                .send("Urgent Warning: You've used up over 90% of your quota");
        }
        else if usage >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        // Interior mutability pattern
        // Allows borrow check at runtime
        // Basically, allows mutating values of immutable references
        sent_messages: RefCell<Vec<String>>
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg));
            
            // The following code panics at runtime
            // let mut first_borrow = self.sent_messages.borrow_mut();
            // let mut second_borrow = self.sent_messages.borrow_mut();

            // first_borrow.push(String::from(msg));
            // second_borrow.push(String::from(msg));
        }
    }

    #[test]
    fn send_over_75_percent_msg() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
