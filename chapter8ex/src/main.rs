use std::collections::HashMap;
use std::io;

fn main() {
    let vec = vec![5, 4, 5, 9, 0, 1];
    let median = median(&vec);
    println!("The median is {median}");

    let mode = mode(&vec);
    if let Some(value) = mode {
        println!("The mode is {value}");
    }
    else {
        println!("There is no mode")
    }

    let str = "apple";
    println!("The Pig Latin of {str} is {}", convert_to_pig_latin(str));

    let str = "first";
    println!("The Pig Latin of {str} is {}", convert_to_pig_latin(str));

    text_interface();
}

fn median(list: &Vec<i32>) -> f64 {
    let mut sorted_list = list.clone();
    sorted_list.sort();

    let len = list.len();
    if len % 2 != 0 {
        sorted_list[list.len() / 2] as f64
    }
    else {
        let first = sorted_list[list.len() / 2];
        let second = sorted_list[(list.len() / 2) - 1];
        (first + second) as f64 / 2.0
    }
    
}

fn mode(list: &Vec<i32>) -> Option<i32> {
    let mut elem_count = HashMap::new();

    for elem in list {
        // Weird asf
        // The key is literally &i32 if I don't use *elem
        let count = elem_count.entry(*elem).or_insert(0);
        *count += 1;
    }

    let (key, value) = elem_count.iter().max_by_key(|entry| entry.1).unwrap();
    if *value == 1 {
        None
    }
    else {
        Some(*key)
    }

}

fn convert_to_pig_latin(str: &str) -> String {
    let first = str.chars().nth(0);
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    if let Some(value) = first {
        if vowels.contains(&value) {
            format!("{str}-hay")
        }
        else {
            format!("{}-{value}ay", &str[1..])
        }
    }
    else {
        String::from("")
    }
}

fn text_interface() {
    let mut registry = HashMap::new();

    loop {
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command");

        let mut args = command.trim().split(' ');
        let args_count = args.clone().count();

        match args_count {
            4 => add_people(&mut registry, &mut args),
            2 => list_people(&mut registry, &mut args),
            1 => {
                let command = args.next().unwrap();
                if command == "quit" {
                    break;
                }
                else {
                    println!("Invalid command: {command}")
                }
            }
            _ => {
                println!("Invalid command: {command}")
            }
        }
    }
}

fn add_people(registry: &mut HashMap<String, Vec<String>>,
              args: &mut std::str::Split<'_, char>) {
    let first = args.next().unwrap();
    if first.to_lowercase() != "add" {
        println!("Invalid command: {first}")
    }

    let name = args.next().unwrap();
    let to = args.next().unwrap();
    if to != "to" {
        println!("Unexpected token: {to}")
    }
    let department = args.next().unwrap();

    let entry = registry
                .entry(department.to_string())
                .or_insert(vec![]);
    // Can't understand why *entry.push() is not valid
    entry.push(name.to_string());
}

fn list_people(registry: &mut HashMap<String, Vec<String>>,
               args: &mut std::str::Split<'_, char>) {
    let first = args.next().unwrap();
    if first.to_lowercase() != "list" {
        println!("Invalid command: {first}")
    }

    let second = args.next().unwrap();
    if second.to_lowercase() == "people" {
        for (key, value) in registry {
            println!("{key}:");
            for people in value {
                println!("{people}");
            }
        }
    }
    else {
        let peoples = registry.get(&second.to_string());
        if let Some(value) = peoples {
            println!("{second}:");
            for people in value {
                println!("{people}");
            }
        }
        else
        {
            println!("Department {second} does not exist")
        }
    }
}
