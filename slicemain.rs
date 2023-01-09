fn main() {
    let numbers = [1, 2, 3, 4, 5];
    
    // create a slice of 2nd and 3rd element
    let slice = &numbers[1..3];
    
    println!("array = {:?}", numbers);
    println!("slice = {:?}", slice);


    //omit the start index
    let slice = &numbers[..3];

    println!("array = {:?}", numbers);
    println!("slice = {:?}", slice);

    //omit the end index
    let slice = &numbers[2..];

    println!("array = {:?}", numbers);
    println!("slice = {:?}", slice);

    //omit both
    let slice = &numbers[..];

    println!("array = {:?}", numbers);
    println!("slice = {:?}", slice);


    //mutable slice
    let mut colors = ["red", "green", "yellow", "white"];
    
    println!("array = {:?}", colors);

    
    let sliced_colors = &mut colors[1..3];
    
    println!("original slice = {:?}", sliced_colors);

    
    sliced_colors[1] = "purple";

    println!("changed slice = {:?}", sliced_colors);

    // slice the string
    let string = String::from("Hello World!");

    
    let slice = &string[0..5];

    println!("string = {}", string);
    println!("slice = {}", slice);


}

