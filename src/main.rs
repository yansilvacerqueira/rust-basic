mod new_module;

use crate::new_module::funcs::get_name;

fn add_number(num: u32) -> u32 {
    return 5 + num
}

// not using return keyword
fn add_number_two(num: u32) -> u32 {
    5 + num
}

fn main() {
    let mut x = 32;
    println!("Hello, world!");
    println!("x is {}", x);

    x = add_number(x);
    println!("now, x is {}", x);

    x = add_number_two(x);
    println!("x2 is {}", x);

    let some_name: &str = "Johanna";
    let name = get_name(some_name);
    println!("name is {}", name);

}

