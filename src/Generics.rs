#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::fmt;

/*
    Using Generics With Functions
    This Function  Will print Anything Long as it implements the Debug Trait
*/
fn Generic_Print<T: fmt::Debug>(x: T) {
    println!("\n{:#?}\n", x);
}

trait Shape<T> {}

#[derive(Debug)]
struct Rect1<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Rect2<U, V> {
    x: U,
    y: V,
}

impl<T> Rect1<T> {
    fn Item(&self) -> &T {
        &self.x
    }
}

// Runner Functions
fn Print_Objects() {
    let R1: Rect1<i32> = Rect1 { x: 12, y: 15 };
    Generic_Print(R1);

    let R1: Rect1<&str> = Rect1 {
        x: "Hello",
        y: "World",
    };
    Generic_Print(R1);

    let R2: Rect2<&str, (i32, i32)> = Rect2 {
        x: "New Rectangle",
        y: (5, 15),
    };
    Generic_Print(R2);
}

pub fn Runner() {
    Print_Objects();
}
