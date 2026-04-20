enum SpreadSheetData {
    Text(String),
    Int(i32),
    Float(f64)
}


fn main() {
    // Declaring a new vector of i32
    let mut v: Vec<i32> = Vec::new();
    // Can do the same with initial values with vec! macro
    // let v = vec![1, 2, 3];
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    // We can read value with either index or get(idx) method
    // 0-based indexing
    let third = v[2];
    // get(idx) returns a Option<&T>
    let third = v.get(2);
    match third {
        Some(&value) => {
            println!("The third element is {value}");
        },
        None => {
            println!("There is no third element");
        }

    }

    let first = &v[0]; // Here we have an immutable reference
    v.push(6); // Here push takes a mutable reference
    // immutable and mutbale reference cannot exists at the same time
    // println!("The first element is {first}");

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        // We use * to dereference
        *i += 5;
        println!("{i}");

        // v.push(*i+1);
        // Again here we have two mutable references one with .push(T) and
        // the one with the for loop
    }

    // We can only store values of same type in a vector
    // But we can use enum to work around since all its variant are of the same type

    let ss_data = vec![
        SpreadSheetData::Text(String::from("Test data")),
        SpreadSheetData::Int(500),
        SpreadSheetData::Float(500.0)
    ];
} // Like any other types with memory on the heap, memory is dropped here for
  // both v and ss_data after their scope ends
