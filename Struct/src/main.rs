fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    /*let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1 //if you want the rest is same as user1,just type this
    };*/
    //let user3 =User("ww","ww.g.com",23,true);
    println!("{} {} {} {}",user1.email,user1.username,user1.active,user1.sign_in_count);
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect2)
    );
    println!("rect2 is {:?}", rect2);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()//call rectangle function :area
    );
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    let sq = Rectangle::square(3);//call other struct function use :: 
    println!("sq {:?}",sq);
}
#[derive(Debug)]//need add this,then Printlm can use {:?}
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle { //like struct personal function
    fn area(&self) -> u32 {//use self because area will know who call this function
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle { //can use as function without self and can let other struct call function 
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn build_user(email: String, username: String) -> User {
    User {
        email, //if field name amd variable name are same,just type once
        username,
        active: true,
        sign_in_count: 1,
    }
}