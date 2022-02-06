#![allow(unused_variables)]
use bFunctions::greet;

fn main() {
    let width = 4;
    let height = 7;
    let depth = 10;

    let area = area_of(width, height);
    let volume = volume_of(width, height, depth);
    println!("Area is {}", area);
    println!("Volume is {}", volume);
    greet();
}

fn area_of(x: i32, y: i32) -> i32 {
    x * y
}

fn volume_of(x: i32, y: i32, z: i32) -> i32 {
    x * y * z
}
