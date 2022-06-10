#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// method for Rectangle
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {

        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );
}

// NOTES

    /* dbg!() */

    // println!("rect1 is {:#?}", rect1);
    // dbg! will always return ownership after
    // The macro 'dbg!' prints to the 
    // std err console stream (stderr)

    /* println! */

    // println!("rect1 is {:#?}", rect1);
    // The fn println! prints to the
    // std out console stream (stdout)

    /* replaced function code */
    
    /*fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
    }*/