fn main() {
    let x = 5 + 5;
    println!("x is = {}", x)
}

// line
/* block */

use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

impl Display for City {
    // 'f' is a buffer, and this method must write the formatted string into it.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 {'N'} else {'S'};
        let lon_c = if self.lon >= 0.0 {'E'} else {'W'};

        // 'write' is like format but it will write the formatted string
        // into a buffer (the first argument)
        write!(f, "{}: {:.3}{} {:.3}{}"),
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    for city in [
        City {name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City {name: "Oslo", lat: 59.95, lon: 10.75 },
        City {name: "Vancouver", lat: 49.25, lon: -123.1},
    ] {
        println!("{}", city);
    }
    for color in [
        Color {red: 128, green: 255, blue: 90 },
        Color {red: 0, green: 3, blue: 254 },
        Color {red: 0, green: 0, blue: 0 },
    ] {
        // Switch this to use {} once you've added an implementation 
        // for fmt::Display
        println!("{:?}", color);
    }
}

// Variables can always be type annoted. Numbers may additionaly be 
// annotated via a suffix or by default. Intergers default to i32 and
// floats to f64. Note that Rust can also infer types from context.

fn main() {
    //variable can be type annoted.
    let logical: bool = true;

    let a_flot: f64 = 1.0; // Regular annotation
    let an_integer = 5i32; //Suffix annotation

    // Or a default will be used
    let default_flot = 3.0; // 'f64'
    let default_integer = 7; //'i32'

    // A type can also be inferred from context
    let mut inferred_type = 12; //Type i64 is inferred from another line.
    inferred_type = 4294967296i64;

    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable 'i32'
    mutable = 21;

    //Error! The Type of a variable cant be changed.
    // mutable = true;
 
    // Variables can be overwritten with shadowing
    let mutable = true;

    /* Compound types - Array and Tuple */

    // Array signature consists of Type T and length as [T; length].
    let my_array: [i32; 5] = [1,2,3,4,5];

    //Tuple is a collection of values of different types
    //and is constructed using parentheses ().
    let my_tuple = (5u32, 1u8, true, -5.04f32);
}


