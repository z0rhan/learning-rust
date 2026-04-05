
fn loop_with_for()
{
    let a = [1, 2, 3, 4, 5];
    for element in a
    {
        println!("{element}");
    }

    // (1..5) is a range from 1 to 5 exclusive
    for number in (1..5).rev()
    {
        println!("{number}");
    }
}


fn loop_with_while()
{
    let mut number = 3;

    while number != 0
    {
        println!("number : {number}");
        number -= 1;
    }
}


fn loop_with_loop()
{
    let mut counter = 0;
    let result = loop
    {
        counter += 1;

        if counter == 10
        {
            break counter * 2;
        }
    };
    println!("The value of result is {result}");

    // label for loops to distinguish for break
    // Interesting
    let mut count = 0;
    'counting_up: loop
    {
        println!("count = {count}");
        let mut remaining = 10;
        loop
        {
            println!("remaining = {remaining}");

            if remaining == 9
            {
                break;
            }
            if count == 2
            {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
}


fn if_expression()
{
    let x = 10;

    // If is an expression
    if x % 2 == 0
    {
        println!("x is even");
    }
    else
    {
        println!("x is odd");
    }
    // Unlike C/C++, the condition must be a bool
    // if x {} gives compilation error
    if x != 0
    {
        println!("x was something other than zero");
    }

    // Multiple conditions with else if
    if x % 2 == 0
    {
        println!("x is divisible by 2")
    }
    else if x % 3 == 0
    {
        println!("x is divisible by 3")
    }
    else if x % 4 == 0
    {
        println!("x is divisible by 4")
    }
    else
    {
        println!("x is not divisible by 2,3 and 4")
    }

    // As mentioned before if is an expression
    if x == 0 { x + 1 } else { x - 1 };
    // Meaning we can bind the result of if to something
    let y = if x == 0 { x + 1 } else { x - 1 };
    println!("The value of y is {y}");
}


fn main()
{
    // if, loop, while and for are expressions and can evaluate to some result
    // if and loop can evaluate to a value
    // for and while only evaluate to unit ()
    // Confusing asf
    loop_with_loop();
    loop_with_while();
    loop_with_for();
}
