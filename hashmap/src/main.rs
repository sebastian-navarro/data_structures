use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    /*
    Second part merge vectors
    */
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10,50];

    let scores:HashMap<_,_> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("Scores: {:?} ", scores);

    // Matching value
    let team_name = String::from("Blue");
    let score = &scores.get(&team_name);
    println!("Score value is: {:?}", score);

    for (key, value) in &scores {
        println!("Key: {} ; Value: {}", key,value);
    }

    // we only can have one key, is not allowed repeated values

    //Classic exercise counting words or characters

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
