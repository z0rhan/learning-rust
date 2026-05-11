fn main() {
    let v1 = vec![1, 2, 3, 4];
    // iterators are lazy
    // By default iterators return immutable ref
    // iter_mut if we want iterators of mutable values
    // into_iter if we want iterators of owned values
    let mut v1_iter = v1.iter();

    'iter: loop {
        // next() cosumes the iterator and returns the value as Option<T>
        if let Some(val) = v1_iter.next() {
            println!("{val}")
        }
        else {
            break 'iter
        }
    }
    // The above loop consumes the iterators
    // We can still use v1_iter but now its at the end of v1
    let _item = v1_iter.next();
    // Here item is just None

    // So we need to make a new one
    let v1_iter = v1.iter();

    // This one takes ownership of the iterator and hence we cannot use the
    // iterator v1_iter after this
    let sum: i32 = v1_iter.sum();

    // This is use of moved value
    // let item = v1_iter.next();
    println!("The sum of {:?} is {sum}", v1);

    // Here map produces new iterator based on the closure provided
    // Tho since iterator are lazy, we have to call methods that consume
    // iterators to get our output, collect() in this case
    let _new_v1: Vec<_> = v1.iter().map(|x| x + 1).collect();
}

#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|shoe| shoe.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
