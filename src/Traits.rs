#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// Traits are Like Interfaces in Java / C++. In Rust, We get many Traits in the **Core Compiler**
use core::ops;

#[derive(Debug, Clone, Copy)]
struct A(f32);

#[derive(Debug)]
struct B(f32);

#[derive(Debug)]
struct AB;

#[derive(Debug)]
struct BA;

#[derive(Debug)]
struct Fib {
    c: u32,
    n: u32,
}
impl Iterator for Fib {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.c + self.n;
        self.c = self.n;
        self.n = n;

        return Some(self.c);
    }
}

impl ops::Add<B> for A {
    type Output = AB;

    fn add(self, _rhs: B) -> AB {
        return AB;
    }
}

// Addition Operator for A & B
impl ops::Add<A> for B {
    type Output = BA;

    fn add(self, _rhs: A) -> BA {
        return BA;
    }
}

// Multiplicaion Operator for A
impl ops::Mul<A> for A {
    type Output = A;

    fn mul(self, _rhs: A) -> A {
        return A(0.0);
    }
}

impl Drop for AB {
    fn drop(&mut self) {
        println!("Dropped this Structure instance");
    }
}

fn Collections_Iterator() {
    let fib = Fib { c: 1, n: 1 };

    for itr in fib.skip(14).take(10) {
        print!("{}\t", itr);
    }
}

fn Structure_Operators() {
    let x1 = A(51.0);
    let x2 = B(22.0);

    println!("Adding A and B together:: {:?}", x1 + x2);

    let x3 = A(12.0);
    println!("Multiplying A and A together:: {:?}", x1 * x3);
}

fn Drop_Trait() {}

fn Clone_Copy() {
    let a = A(51.0);
    println!("{:?}", a);

    println!("Clone and Copy gets called here");
    let c = a;
    println!("{:?}", a);
}

pub fn Runner() {
    println!("\nClone_&_Copy_Trait");
    Clone_Copy();

    println!("\nOverloading default Operators for Structures");
    Structure_Operators();

    println!("\n\n");
    Collections_Iterator();

    println!("\n\n");
}
