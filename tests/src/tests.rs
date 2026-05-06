use crate::quadrilateral::Quadrilateral;

#[test]
fn can_hold_smaller() {
    let larger = Quadrilateral::new(5, 6);
    let smaller = Quadrilateral::new(4, 5);

    // Expects a bool and asserts it is true
    // In case it is false, calls panic!
    assert!(larger.can_hold(&smaller));
}

#[test]
fn cannot_hold_larger() {
    let larger = Quadrilateral::new(5, 6);
    let smaller = Quadrilateral::new(4, 5);

    assert!(!smaller.can_hold(&larger));
}

#[test]
fn merge_dimensions() {
    let mut quad1 = Quadrilateral::new(5, 6);
    let quad2 = Quadrilateral::new(4, 5);

    quad1.merge(quad2);

    // Useful when asserting two values as equal
    // In case it is not equal, panic! with two value printed
    assert_eq!(*quad1.length(), 9);
    // We can also pass custom message here
    // Any arguments after the required arguments are passed to format! macro
    assert_eq!(*quad1.width(), 11,
               "The width should be 11: 5 + 6");
}

#[test]
fn merge_new_dimensions() {
    let quad1 = Quadrilateral::new(5, 6);
    let quad2 = Quadrilateral::new(4, 5);

    let quad3 = quad1.merge_to_new(quad2);

    // Useful when asserting two values as not equal
    // In case it is not not equal, panic! with two value printed
    assert_ne!(*quad3.length(), 5);
    assert_ne!(*quad3.length(), 4);

    assert_ne!(*quad3.width(), 6);
    assert_ne!(*quad3.width(), 5);
}

#[test]
// This attribute makes the test pass if the code inside panics
// Here we can pass an expected parameter which is check against the panic message
#[should_panic(expected = "100x100")]
fn new_invalid_dimenstion() {
    Quadrilateral::new(101, 101);
}

// We can also use Result with tests
// Then, the value in Err(T) is printed if the test fails and continues
// whereas with assert*, the program would stop immediately
// Also, it means the value in Err() should be printable
#[test]
fn length_correct() -> Result<(), String> {
    let quad1 = Quadrilateral::new(5, 6);

    if *quad1.length() == 5 {
        Ok(())
    }
    else {
        Err(String::from("Length from getter does not match"))
    }
}
