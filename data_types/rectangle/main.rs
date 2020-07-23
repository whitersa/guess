// basic version
fn main () {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area (width: u32, height: u32) -> u32 {
    width * height
}


// tuple version
fn main () {
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}
fn area (dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// struct version 
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main () {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:#?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1) 
    );
}

fn area (rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// decoupling
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main () {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:#?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

// in other language, -> for pointers and . for direct object
object->something() // if object is a pointer then
(*object).something()
// rust will automatical referencing and dereferencing
// that is to say,  when you call a method with object.something()
// rust automatically adds in &, &mut, or *
// p1.distance(&p2);   (&p1).distance(&p2)    



// one more methods
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // fn area(&self) -> u32 {
    //     self.width * self.height
    // }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
fn main () {
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

    println!("Can rect1 hold rect2 ?{}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3 ?{}", rect1.can_hold(&rect3));
}

// associated functions
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}