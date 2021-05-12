
fn main(){
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert(String::from("key1"), 1);
    map.insert(String::from("key2"), 2);
    /*let map = vec![
    (String::from("key1"), 1),
    (String::from("key2"), 2),
    ].into_iter().collect::<HashMap<_, _>>();*/
    // 這邊的鍵要用 borrow 型態的，所以用 str 也是可以的
    // 如果沒有這個鍵的話會 panic
    println!("{}", map["key1"]);
    // 這個做法的話會回傳 Option
    println!("{:?}", map.get("key3"));
    map.insert(String::from("key1"), 3);
    //need mp be mutable
    }