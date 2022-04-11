#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

/*
    This File also includes Box Pointers
    Closures are Like Delegates(Function Pointers) in C#.
    More closely like ArrowFunctions in JavaScript.
    ! DONT GO CREATE CLOSURES ALL THE TIME...
    ! BEST PLACE TO PASS/USE CLOSURE IS INSIDE PREBUILT LIBRARY FUNCTIONS LIKE MACROS, ITERATORS <=|=> Lambda Functions
*/

#[derive(Debug)]
struct MathFuncs<F1, F2>
where
    F1: Fn(i32) -> i32,
    F2: Fn(),
{
    AddFn: F1,
    subFn: F2,
}

fn create() -> Box<dyn Fn()> {
    Box::new(move || {
        println!("This is a Cool way of defining Function... But its pretty difficult!!")
    })
}

fn run<F: Fn(i32) -> i32>(f: F) -> i32 {
    let x = f(10);
    x
}

fn ClousersRunner<F>(f: F)
where
    F: Fn(),
{
    f();
}

// Runner Functions
fn Iterators_n_CLosures() {
    let v1: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    println!("?? => {}", v1.iter().any(|&x: &i32| x == 4));
}

fn Closures_Runner() {
    let f = |i: i32| i * 10;
    print!("{}", f(15));

    let pr = || println!("Just a Random Clousure");

    println!("{}", run(f));

    ClousersRunner(pr);

    // Random Useless CL!!
    let newcl = |a: i32, b: i32, c: i32| -> (i32, i32, i32) { (0, 0, 0) };

    // This is Good Implementation :)!!
    // Here we use Generics and CLosures to Store 2 functions into a Struct
    let newformula = MathFuncs {
        AddFn: f,
        subFn: pr,
    };

    let fnc = create();
    fnc();
}

fn Box_Pointer_Impl() {
    let x1: Box<i32> = Box::new(10);
    let x2: Box<f32> = Box::new(25.02);

    println!("{}", x1);
    println!("{}", x2);
}

pub fn Runner() {
    // Box_Pointer_Impl();
    // println!();

    // Closures_Runner();

    Iterators_n_CLosures();
}
