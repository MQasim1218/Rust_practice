#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_macros)]

use std::collections::HashMap;

/*
    Macros are Code Snippits in rust that can be used to avoid repitition.
    Println! and Vec! are both macros.
    Macros expand to actual code at compile-time.
    Can take any number of arguments.
    Supports Overloading of internal functions via different parameter combinations
*/

// Keyword for defining Macros
macro_rules! new_macro {
    () => {
        println!("This is a Macro");
    };
}

/*
    ! MACRO_RULES Syntax
    (parameter => $name : type)

    ! USAGE
    Parameter -> Optional
    $ -> must
    name -> User-defined name for parameter
    type -> identifier for incoming value
    [
        ident -> identiity,
        expr -> expression,
        block -> Code Block (Closure),
        item,
        path,
        pat -> pattern,
        tt -> token_tree,
        stmt -> statement,
        ty -> type
    ]
*/
macro_rules! x_y {
    (x => $e:expr) => {
        println!("X: {}", $e);
    };

    (y => $e:expr) => {
        println!("Y: {}", $e);
    };
}

// This is a Bad Example to declare Static functions but cna be helpful when working with Closures.
macro_rules! new_func {
    ($funcName: ident) => {
        fn $funcName() {
            println!("You Called {:?}()", stringify!($funcName));
        }
    };
}

macro_rules! print_exp {
    ($e: expr) => {
        println!("{:?} = {:?}", stringify!($e), $e);
    };
}

macro_rules! exame {
    ($exp1: expr, && $exp2: expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($exp1),
            stringify!($exp1),
            $exp1 && $exp2
        );
    };

    ($exp1: expr, || $exp2: expr) => {
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($exp1),
            stringify!($exp1),
            $exp1 || $exp2
        );
    };
}

// This isn't working due to some reason
#[macro_export]
macro_rules! even_odd_list_generator {
    // id1, id2 are identities where(|) <-  [range {$start, $end}], $condition: expr
    ($id1: ident | $id2: ident <== [$start: expr, $end: expr], $cond: expr ) => {
        {
            let mut vec = Vec::new();
            for num in $start..$end + 1 {
                if $cond(num) {
                    vec.push(num);
                }
            }
            vec;
        }
    };
}

macro_rules! new_map {
    ($($key: expr => $val: expr),*) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($key, $val);
            )*

            map
        }
    };
}

fn evens(x: i32) -> bool {
    return x % 2 == 0;
}

pub fn Runner() {
    /*
        new_macro!();
        x_y!(x => 10);
        x_y!(y => 20 + 30);
        new_func!(Hello_There);
        Hello_There();
    */

    /*
        print_exp!({
            let y = 20;
            let z = 50;
            let a = 12 + 9 - 15;
            z + y * a
        })
    */

    /*
        exame!(1 == 1, &&2 == (1 + 1));
        exame!(1 == 3, || 2 == (1 + 1));
    */

    let evens = even_odd_list_generator!(x | x <== [1, 10], evens);
    println!("{:?}", evens);
    let odds = even_odd_list_generator!(x | x <== [1, 10], (|x| x % 2 != 0));
    println!("{:?}", odds);
    println!("Over here");

    let m = new_map! {
        1 => "A",
        2 => "B"
    };

    println!("{:#?}", m)
}
