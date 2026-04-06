fn main() {
    let mut x = 1;

    while x < 10
    {
        let fibonnaci_num = fibonnaci_with_recursion(x);
        println!("The {x}th fibonnaci number is {fibonnaci_num}");
        x += 1;
    }

    while x < 15
    {
        let fibonnaci_num = fibonnaci_with_loop(x);
        println!("The {x}th fibonnaci number is {fibonnaci_num}");
        x += 1;
    }

    let temperature = 38.0;
    let fahrenheit = celsius_to_fahrenheit(temperature);
    println!("Celsius {temperature} in Fahrenheit is {fahrenheit}");
    let celsius = fahrenheit_to_celsius(temperature);
    println!("Fahrenheit {temperature} in Celsius is {celsius}");

}


fn fibonnaci_with_recursion(n: i32) -> i32
{
    if n == 1
    {
        0
    }
    else if n == 2
    {
        1
    }
    else
    {
        // This way of returning things is quite unique
        fibonnaci_with_recursion(n-1) + fibonnaci_with_recursion(n-2)
    }
    // Also, if without else evaluates to ()
    // Makes sense if you think about as when you want to bind to say 'x'
    // If the condition of if evaluates to false then what is 'x'?
    // So else is needed otherwise the resultant is just ()
}

fn fibonnaci_with_loop(n: i32) -> i32
{
    let mut first = 0;
    let mut second = 1;

    if n == 1
    {
        first
    }
    else if n == 2
    {
        second
    }
    else
    {
        let mut count = 2;
        let mut result = first + second;

        while count < n
        {
            result = first + second;
            first = second;
            second = result;
            count += 1;
        }

        result
    }
}

fn celsius_to_fahrenheit(celsius: f64) -> f64
{
    // so floats have to be explicitly declared with .0
    celsius * (9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64
{
    (fahrenheit - 32.0) * (5.0 / 9.0)
}
