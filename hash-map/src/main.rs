use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Red"), 10);
    scores.insert(String::from("Blue"), 20);
    scores.insert(String::from("Yellow"), 15);

    // We can acess the value with .get(K)
    let team = String::from("Black");

    let score = scores.get(&team); // Returns a Option<&V>
    match score {
        Some(&value) => {
            println!("The score of team {team} is {value}");
        },
        None => {
            println!("The team {team} has no score");
        }
    }

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Here since team: String has not implemented Copy, scores takes the
    // ownership of team
    scores.insert(team, 10);
    // println!("{team}"); -> Use of moved value

    updating_hash_map();
}

fn updating_hash_map() {
    // We can completely disregard the old value and overwrite it
    let mut scores = HashMap::new();
    scores.insert(String::from("Red"), 10);
    print_hasp_map(&scores);
    scores.insert(String::from("Red"), 15);
    print_hasp_map(&scores);
    // insert for the same key overwrites the original value

    // Lets say we only want to insert if the key does not exist, we have entry for that
    scores.entry("Yellow".to_string()).or_insert(5);
    scores.entry("Red".to_string()).or_insert(15);
    print_hasp_map(&scores);

    // or_insert() also returns a mutable reference which we can use to update the value
    let score = scores.entry("Red".to_string()).or_insert(0);
    *score += 5;
    print_hasp_map(&scores);
}

fn print_hasp_map(scores: &HashMap<String, i32>) {
    for (k, v) in scores {
        println!("{k}: {v}");
    }
}
