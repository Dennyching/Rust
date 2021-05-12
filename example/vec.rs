fn main(){
    let v = vec![1, 2, 3];

    // 這邊跟陣列一樣從 0 開始，若取超過範圍會 panic
    let n = v[1];

    // 這邊回傳的是 Option<&i32> ，若超過範圍只會回傳 None
    let n1 = v.get(1);
    println!("{:?} {} {:?}",v ,n ,n1);
    match n1 {
        Some(x) =>println!("{:?} ",x),
        None =>println!("Nothing"),
    };
    let array = [1, 2, 3];
    let mut vec = Vec::new();

    // 這邊的 i 型態是 &i32 iter vector
    for i in array.iter() {
    vec.push(i * 2);
    }

    println!("{:?}", vec);
    let array = [1, 2, 3];
    // Vec<_> 可以這樣寫是因為中間的型態可以讓編譯器自動推導
    //let vec = array.iter().map(|x| x * 2).collect::<Vec<_>>();
    // 如果你真的不喜歡 ::<Vec<_> 的語法也可以改用型態標註
    //map 每個元素都用其中的函式做轉換，再產生一個新的迭代器
    //collect 迭代器的值再搜集變成某個集合型態
    let vec: Vec<_> = array.iter().map(|x| x * 2).collect();
    println!("{:?}", vec);
    // 這邊就像一般函式一樣要放型態了
    fn time2(x: &i32) -> i32 { x * 2 }

    let array = [1, 2, 3];
    let vec = array.iter().map(time2).collect::<Vec<_>>();
    println!("{:?}", vec);
    //range .. 本來就是Iterator 可以直接call filter 來找偶數
    println!("{:?}", (1..=10).filter(|x| x % 2 == 0).collect::<Vec<_>>());
    let array = [1, 2, 3];
    // iter 中必須要記錄目前跑到哪個值，所以必須是 mut
    let mut iter = array.iter();
    //next will call next element in Some(x) format
    println!("{:?}", iter.next()); // => Some(&1)
    println!("{:?}", iter.next()); // => Some(&2)
    println!("{:?}", iter.next()); // => Some(&3)
    // 已經沒有值了
    println!("{:?}", iter.next()); // => None
    // 沒有值後再繼續呼叫並不會錯誤，而是一直回傳 None
    println!("{:?}", iter.next()); // => None

}