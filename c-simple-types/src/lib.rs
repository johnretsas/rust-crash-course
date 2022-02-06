pub fn print_difference(coords: (f32, f32)) -> f32 {
    let diff = coords.0 - coords.1;
    println!(
        "The difference between {} and {} is {}",
        coords.0,
        coords.1,
        diff.abs()
    );
    diff
}
pub fn print_array(a: [f32; 2]) {
    println!("The coordinates are ({}, {})", a[0], a[1]);
}

pub fn ding(x: i32) {
    if x == 13 {
        println!("Ding, you found 13!");
    }
}

pub fn on_off(value: bool) {
    if value == true {
        println!("Lights are on!")
    }
}

pub fn print_distance(z: (f32, f32)) {
    println!(
        "Distance to the origin is {}",
        (z.0.powf(2.0) + z.1.powf(2.0)).sqrt()
    );
}
