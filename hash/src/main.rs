fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Yellow"), 25);//replace yellow 50
    scores.entry(String::from("Yellow")).or_insert(50);
    //only insert value if key don't have value
    scores.entry(String::from("Green")).or_insert(50);

    let team_name = String::from("Blue");
    let _score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    
    //update value
    println!("{:?}", map);

}
