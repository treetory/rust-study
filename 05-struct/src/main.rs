fn main() {
    let length1 = 50;
    let width1 = 30;

    struct_example_5_7(length1, width1);

    struct_example_5_8(length1, width1);

    struct_example_5_9(length1, width1);

    struct_example_5_12(length1, width1);

    struct_example_5_13();

    struct_example_5_14(3);
}

//////////////////////////     구조체     //////////////////////////
// use variable
fn struct_example_5_7 (length1: u32, width1: u32) {

    println!(
        "The area of the rectangle is {} square pixels.", area_1(length1, width1)
    )
}

fn area_1(length: u32, width: u32) -> u32 {
    length * width
}

// use tuple
fn struct_example_5_8 (length1: u32, width1: u32) {
    println!(
        "The area of the rectangle is {} square pixels.", area_2((length1, width1))
    )
}

fn area_2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// use struct
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn struct_example_5_9 (length1: u32, width1: u32) {

    let rect1 = Rectangle {
        length: length1, width: width1
    };

    println!(
        "The area of the rectangle is {} square pixels.", area_3(&rect1)
    );

    //println!("rectangle is {}", rect1);
    // -> error: the trait bound 'Rectangle: std::fmt::Display' is not satisfied

    println!("rectangle is {:?}", rect1);
    // -> error: the trait bound `Rectangle: std::fmt::Debug` is not satisfied
    // -> #[derive(Debug)] 어노테이션 추가 후, 동작
    println!("rectangle is {:#?}", rect1);
    // -> beautify the print

}

fn area_3(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}

// use method
impl Rectangle {

    fn area_method(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

}

fn struct_example_5_12 (length1: u32, width1: u32) {

    let rect1 = Rectangle {
        length: length1, width: width1
    };

    println!(
        "The area of the rectangle is {} square pixels.", rect1.area_method()
    );

}

fn struct_example_5_13 () {
    let rect1 = Rectangle {length: 50, width: 30};
    let rect2 = Rectangle {length: 40, width: 10};
    let rect3 = Rectangle {length: 45, width: 60};

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

// associated functions
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {length: size, width: size }
    }
}

fn struct_example_5_14 (size: u32) {
    let sq = Rectangle::square(size);
    println!("square is {:?}", &sq);
}
