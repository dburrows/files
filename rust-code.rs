// below is automatically inserted
// use std::prelude::v1::*;

// how to define const values with functions
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref A: f64 = 1.0 / (2.0_f64.sqrt());
    static ref B: f64 = 1.0 / 10.0_f64.sqrt();
    static ref C: f64 = 1.0 / 42.0_f64.sqrt();
}

// or use a const fn

const fn inc(n: i32) -> i32 {
    n + 1
}
const BAR: i32 = inc(4);

fn main() {
    println!("Hello, world!");
    println!("{}", *A);
    println!("{}", BAR);

    fn fair_dice_roll_noreturn(feeling_lucky: bool) -> i32 {
        if feeling_lucky {
            6
        } else {
            4
        }
    }

    fn fair_dice_roll(feeling_lucky: bool) -> i32 {
        match feeling_lucky {
            true => 6,
            false => 4,
        }
    }

    // below won't work, consts are evaluated at compile time
    // const fdrnr: i32 = fair_dice_roll_noreturn(true);
    let fdrnr: i32 = fair_dice_roll_noreturn(true);
    let fdr: i32 = fair_dice_roll(true);

    println!("fair dice roll no return: {}", fdrnr);
    println!("fair dice roll: {}", fdr);

    let _least = std::cmp::min(3, 8);

    struct Vec2 {
        x: f64, // 64-bit floating point, aka "double precision"
        y: f64,
    }

    let _v1 = Vec2 { x: 1.0, y: 3.0 };
    let v2 = Vec2 { y: 2.0, x: 4.0 }; // order doesn't matter

    let v3 = Vec2 { x: 14.0, ..v2 }; // kind of spread operator
    let _v4 = Vec2 { ..v3 };

    let v = Vec2 { x: 3.0, y: 6.0 };
    let Vec2 { x, y } = v; // destructure
    let Vec2 { x: newx, y: newy } = v; // destructure & rename
    let Vec2 { x: newerx, .. } = v;
    // `x` is now 3.0, `y` is now `6.0`

    // this throws away `v.y`

    println!("{} {}", v.x, v.y);
    println!("{} {}", x, y);
    println!("{} {}", newx, newy);
    println!("{}", newerx);

    // let patterns can be used as conditions in if:
    struct Number {
        odd: bool,
        value: i32,
    }

    fn print_number(n: Number) {
        if let Number { odd: true, value } = n {
            println!("Odd number: {}", value);
        } else if let Number { odd: false, value } = n {
            println!("Even number: {}", value);
        }
    }

    let one = Number {
        odd: true,
        value: 1,
    };
    let two = Number {
        odd: false,
        value: 2,
    };
    print_number(one);
    print_number(two);
    // this prints:
    // Odd number: 1
    // Even number: 2

    // fn print_number_match(n: Number) {
    //     match n {
    //         Number { odd: true, value } => println!("Odd number: {}", value),
    //         Number { odd: false, value } => println!("Even number: {}", value),
    //     }
    // }
    // print_number_match(one);
    // print_number_match(two);
    // this prints the same as before

    // match must be exhaustive
    // _ can be used as a catch all pattern
    fn print_number_default_match(n: Number) {
        match n.value {
            1 => println!("One"),
            2 => println!("Two"),
            _ => println!("{}", n.value),
        }
    }
}
