use std::thread;

#[derive(Debug)]
struct Quad {
    width: u32,
    height: u32
}

fn main() {
    let mut store = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red]
    };

    let user1_pref = Some(ShirtColor::Blue);
    let giveaway1 = store.giveaway(user1_pref);
    println!("User with preference {:?} gets {:?}", user1_pref, giveaway1);


    let user2_pref = None;
    let giveaway2 = store.giveaway(user2_pref);
    println!("User with preference {:?} gets {:?}", user2_pref, giveaway2);

    // This does immutable capture of store variable
    let print_shirts = || println!("The Inventory has {:?}", store.shirts);
    print_shirts();

    // This does mutable capture of store variable
    let mut add_blue = || {
        store.add_blue();
        println!("Added 1 blue shirt");
    };
    add_blue();
    store.print_shirts();

    // This moves the ownership of store to the new thread
    // Here the main thread might finish before this new thread and drop store
    // Hence, it is better to move store and also enforced by the compiler
    // unless store has static lifetime
    thread::spawn(move || {
        println!("In another thread:");
        store.print_shirts();
    }).join().expect("Something went wrong in new thread");


    let mut list = [
        Quad {width: 10, height: 1},
        Quad {width: 3, height: 5},
        Quad {width: 7, height: 12}
    ];

    // let mut sort_ops = vec![];
    // let annouce = String::from("Closure called");
    let mut ops_count = 0;

    // This is the FnMut trait closure which is called multiple times
    list.sort_by_key(|q| {
        // This moves annouce which is captured out of its body
        // Meaning it can only be called once so a FnOnce
        // But sort_by_key needs FnMut closure
        // sort_ops.push(annouce);
        ops_count += 1;
        q.width
    });
    println!("{:?}", list);
}

fn comparison_fn_closure() {
    let add_one_v2 = |x: i32| -> i32 { x + 1 };
    let add_one_v3 = |x| { x + 1 };
    let add_one_v4 = |x|  x + 1 ;

    let x = 1;
    let _x_added_1 = add_one_v1(x);
    let _x_added_1 = add_one_v2(x);
    let _x_added_1 = add_one_v3(x);
    let _x_added_1 = add_one_v4(x);


    // let x_added_1 = add_one_v4("string");
    // The above is not valid since the first use of add_one_v4 set its
    // parameter type to i32 and now we are trying to use &str
}
fn add_one_v1(x: i32) -> i32 { x + 1 }


#[derive(Debug, Clone, Copy, PartialEq)]
enum ShirtColor {
    Red,
    Blue
}

#[derive(Debug)]
struct Inventory {
    shirts: Vec<ShirtColor>
}

impl Inventory {
    fn giveaway(&self, user_pref: Option<ShirtColor>) -> ShirtColor {
        user_pref.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut red_num = 0;
        let mut blue_num = 0;

        for shirt in &self.shirts {
            match shirt {
                ShirtColor::Red => red_num += 1,
                ShirtColor::Blue => blue_num += 1
            }
        }

        if red_num > blue_num {
            ShirtColor::Red
        }
        else {
            ShirtColor::Blue
        }
    }

    fn add_blue(&mut self) {
        self.shirts.push(ShirtColor::Blue);
    }

    fn print_shirts(&self) {
        println!("{:?}", self.shirts);
    }
}

// unwrap_or_else implementation
//  pub const fn unwrap_or_else<F>(self, f: F) -> T
//  where
//      F: [const] FnOnce() -> T + [const] Destruct,
//  {
//      match self {
//          Some(x) => x,
//          None => f(),
//      }
//  }

// All closure implements FnOnce traits which means the closure can be called
// once, closure that can move out captured value will only implement this trait
//
// FnMut applies to closure that don't move captured value out but can mutate
// them, can be called multiple times
//
// Fn applies to closure that don't move captured values out, don't mutate
// captured values as well as closure that don't capture anything
//
// We can also prove the function name in case the closure does not need to
// capture any variable, then the compiler implements the necessary Fn trait
