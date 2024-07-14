fn main() {


    // Exercise A Variables
    let missiles = 8;
    let ready = 2;
    println!("Firing{} of my {} missiles...", ready, missiles);
    let width =4;
    let length = 5;
    let depth =6;

    // Exercise B  Functions
    println!("{}", area(width, length, depth));

    // Exercise C types
    let coordinates = (4.0, 5.0, 6.0);
    let coordArray = [coordinates.0, coordinates.1, coordinates.2];
    println!("{:?}", coordArray);

    // Exercise D types
    for num in coordArray.iter(){
        println!("{}", num);
    }
    for num in 0..50{
        println!("{}", num);
    }


}


fn area(width: u32, length: u32, depth: u32) -> u32 {
    return width * length * depth
}

