#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U
}

// Enum can also be generic over some type T
// E.g. Option<T> is just
// enum Option<T> {
//     Some(T),
//     None
// }
// Another example is Result<T, E>
// enum Result<T, E> {
//     Ok(T),
//     Err(E)
// }

// Generics on methods
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<T, Y2> {
        Point { x: self.x, y: other.y }
    }
}

// We can restrict the methods on the types by,
impl Point<f32, f32> {
    fn is_float(&self) -> bool {
        true
    }
}

fn main() {
    let num_list = vec![55, 40, 100, 33, 67];
    let result = largest(&num_list);
    println!("The largest num of {num_list:?} is {result}");

    let num_list = vec![7, 3, 2, 4, 9, 0, 1];
    let result = largest(&num_list);
    println!("The largest num of {num_list:?} is {result}");

    let p_int = Point { x: 1, y: 2 };
    println!("p is {p_int:?}");
    let p_float = Point { x: 1.0, y: 2.0 };
    println!("p is {p_float:?}");
    let p = Point { x: 1.0, y: 2 };
    println!("p is {p:?}");

    println!("p has x: {}", p.x());
    println!("p has y: {}", p.y());

    if p_float.is_float() {
        println!("p_float is float");
    }
    // Not defined for anything except Point<f32, f32>
    // if p_int.is_float() {
    //     println!("p_float is float");
    // }

    let new_p = p.mixup(p_float);
    println!("new_p is {new_p:?}");
}

fn largest<T: std::cmp::PartialOrd>(list: &Vec<T>) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
