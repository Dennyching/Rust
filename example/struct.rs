#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}
fn rect_area(rectangle : Rectangle) ->f32{
    let mut area = (rectangle.bottom_right.x-rectangle.top_left.x)*(rectangle.bottom_right.y-rectangle.top_left.y);
    if  area<0.0{
        area=area*-1.0;
    }
    area

}
fn square(point:Point,heigh:f32)->Rectangle{
    let point1= Point{
        x:point.x+heigh,
        y:point.y+heigh,
    };
    let rect = Rectangle{
        top_left: point,
        bottom_right: point1,
    };
    rect
}
// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);


    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };//will get point.y

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: top_edge, y: left_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", _rectangle.bottom_right, _rectangle.top_left);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("area = {}",rect_area(_rectangle));
    let rect_square = square(point, 4.0);
    println!("pair contains {:?} and {:?}",rect_square.bottom_right, rect_square.top_left);

}
