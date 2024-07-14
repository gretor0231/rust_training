fn main() {


    // ExerciseA Variables
    let missiles = 8;
    let ready = 2;
    println!("Firing{} of my {} missiles...", ready, missiles);


    let width =4;
    let length = 5;
    let depth =6;

    // ExerciseB Functions
    println!("{}", area(width, length, depth));
}


fn area(width: u32, length: u32, depth: u32) -> u32 {
    return width * length * depth
}

