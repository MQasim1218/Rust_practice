#[allow(non_snake_case)]
mod Closures;
/*
    mod Generics;
    mod Traits;
    mod control_flow__ifelse__loop;
    mod structs_traits_funtions;
*/

fn main() {
    Closures::Runner();
    // Generics::Runner();
    // Traits::Runner();

    // structs_traits_funtions::runner();
    /*
        // Arrays and Slices
        let arr: [i64; 10] = [1, 2, 3, 4, 6, 5, 7, 8, 9, 0];
        println!("This is the length of the array :: {}", arr.len());
        let slice = &arr[2..4];
        println!("{:?}", slice);
    */
    //     for chars in &arr[1..5] {
    //         print!("{}\n", chars);
    //     }

    //     let str_slice = "This isnt a String.. It is a slice of a string";
    //     let str = str_slice.to_string();

    //     let name: String = String::from("Qasim");
    //     println!("{}", name);

    //     let concat_strings = str + ",  " + &name;
    //     println!("{}", concat_strings);
    // }
}
