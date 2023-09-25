use core::fmt;
use std::f64::consts::PI;

#[derive(Debug)]


enum Shape {
    Circle(f64),
    Rectangle(f64,f64),
    Triangle(f64,f64,f64)
}

impl Shape {
    fn get_perimeter(&self) -> f64 {
        match *self {
            Shape::Circle(r) => 2.0 * std::f64::consts::PI * r,
            Shape::Rectangle(l,w ) => (2.0 * l) + (2.0 * w),
            Shape::Triangle(s1,s2, s3 ) => s1 + s2 + s3
        }
    }
}

enum Location {
    Anonymous,
    Unknown,
    Known(f64, f64)
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match *self {
            Location::Anonymous => write!(f, "Cannot tell you where we are "),
            Location::Unknown => write!(f, "I do not know where we are"),
            Location::Known(lon, lat ) => write!(f, "We are at lattitude {}, longitude {}", lat, lon)
        }
    }
}

impl Location {
    fn display(&self) {

        match *self {
            Location::Unknown => println!("Don't know where we are"),
            Location::Anonymous => println!("Not telling you where we are"),
            Location::Known(lon,lat ) => println!("We are at lattitude {}, longitude{}", lon, lat)
        }

    }
}

fn main() {
    let my_shape = Shape::Circle(1.2);
    println!("My shaoe is {:?} !", my_shape);

    match my_shape {
        Shape::Circle(r) => println!("This shape is a {:?} with a radius of {}", my_shape,r),
        Shape::Rectangle(s,t ) => println!("This shape is a {:?}, with sides of {} and {}", my_shape, s, t),
        Shape::Triangle(u, v,w ) => println!("This shape is a {:?}, with sides of {}, {}, and {}", my_shape, u, v,w)
    }

    let my_number = 8u8;
    let result = match my_number {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        _ => {
            println!("Number doesn't match {}", my_number);
            "something else"
        }
    };

    println!("The restul is {}", result);

    let perimeter = my_shape.get_perimeter();
    println!("The shape perimeter is {}", perimeter);

    let countdown = [5,4,3,2,1];
    let number = countdown.get(5);
    let number = match number {
        Some(number) => number + 1,
        None => 0
    };
    println!("number is {:?}", number);

    let number = Some(13);
    match number {
        Some(13) => println!("thirteen"),
        _ => ()
    }

    if let Some(13) = number {
        println!("thirteen");
    }

    let address = Location::Unknown;
    address.display();
    println!("The location is {}", address);
    let address = Location::Anonymous;
    address.display();
    let address = Location::Known(54.8, 32.1);
    address.display();
}
