fn main() {
    let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The max config is {max}"),
    //     _ => ()
    // }
    // Here, _ => () is annoying boilerplate
    // The above can be achieved with if let as,

    if let Some(max) = config_max {
        println!("The max config is {max}")
    }
    // Syntax sugar for match which runs for matched pattern and ignores everything else
    // We can also add else branch to it
    else {
        println!("The max config is not given")
    }

    let analysis = analyze_config(&config_max);

    if let Some(value) = analysis {
        println!("{}", value);
    }

}

fn analyze_config(config: &Option<u8>) -> Option<String> {
    // if let &Some(max) = config {
    //     if max < 100 {
    //         Some(String::from("The max config is too low"))
    //     }
    //     else if max < 200 {
    //         Some(String::from("The max config is perfect"))
    //     }
    //     else {
    //         Some(String::from("The max config is too high"))
    //     }
    // }
    // else {
    //     None
    // }
    // Above is hard to follow as it becomes more complex

    // let max_config = if let &Some(max) = config {
    //     max
    // }
    // else {
    //     return None;
    // };

    // if max_config < 100 {
    //     Some(String::from("The max config is too low"))
    // }
    // else if max_config < 200 {
    //     Some(String::from("The max config is perfect"))
    // }
    // else {
    //     Some(String::from("The max config is too high"))
    // }
    // This is not any better
    // Can use let..else here

    // If pattern matches, binds the value in outer scope
    // else executes the else branch
    let &Some(max) = config else {
        return None;
    };

    if max < 100 {
        Some(String::from("The max config is too low"))
    }
    else if max < 200 {
        Some(String::from("The max config is perfect"))
    }
    else {
        Some(String::from("The max config is too high"))
    }
}
