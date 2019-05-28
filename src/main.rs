fn main() {
    let number = define_number(8);
    println!("Our number is {}", number);

    // IF

    // Blocks of code executed in if / else statements are called 'arms'
    // Conditions MUST be booleans (no type conversion)
    // This doesn't work (unlike in JS) :
    // if number {
    if number < 10 {
        println!("The number is indeed smaller than 10");
    } else {
        println!("The number is not smaller than 10");
    }

    // if is an expression (returns value)
    // Can be assigned to variable
    let condition = get_condition(number);
    let number_height = if condition {
        "higher than 10"
    } else {
        "lower than 10"
        // Arms must returns values of the same types
        // Because variables must have a single type
        // This is invalid :
        // number
    };

    println!("Our number is {}", number_height);

    // LOOP

    // loops forever until explicitly told to stop with break
    // loop {
    //     println!("again")
    // }
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            // break keyword takes an optional value to return after breaking the loop
            // here it is counter * 2
            break counter * 2;
            // break alone returns an empty tuple (I think)
        }
    };
    println!("The result is {}", result);


    // WHILE

    let mut number = 3;
    // while condition is true, run code in block
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
    println!("LIFTOFF!!!");

    // FOR
    // Most commonly used loop because of safety and conciseness
    let animals = ["Lion", "Turtle", "Frog"];

    // .iter() looks like a method on arrays
    // Tooltip says 'returns iterator over the slice'
    for animal in animals.iter() {
        println!("This is an animal : {}", animal)
    }

    // liftodff while loop with a range and a for loop
    // range syntax => (min_bound(inclusive)..max_bound(exclusive))
    // Another array method, .rev() 'Reverse an iterator direction
    for number in (0..6).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

}


fn define_number(num: i32) -> i32 {
    num
}

fn get_condition(num: i32) -> bool {
    num > 10
}
