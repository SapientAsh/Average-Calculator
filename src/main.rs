use std::env;
use std::process::exit;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    //Make sure arguments have been provided
    if args.len() < 2 { 
        println!("Please provide at least one number");
        exit(1);
    }

    let mut numbers: Vec<i32> = Vec::new();

    //Convert all valid arguments to integers
    for i in 1..args.len() {
        let arg: i32 = match args[i].trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        //Push valid arguments to vector
        numbers.push(arg);
    }
    
    //Make sure valid arguments have been provided
    if numbers.len() < 1 {
        println!("No valid arguments");
        exit(1);
    }

    //Calculate mean
    ////Iterates over numbers, summing contents, divides by length of the vector
    println!("The mean is {}", numbers.iter().sum::<i32>() as f64 / numbers.len() as f64);

    //Calculate mode
    ////Creates a hashmap, attempts to find each element as a key in the map and add 1 to the
    ////value, else adds new key with value of 1
    let mut map = HashMap::new();
    for i in 0..numbers.len() {
        let val = map.entry(numbers[i]).or_insert(0);
        *val += 1;
    }

    ////Finds the key in the map with the maximum value
    let mut ind = 0;
    let mut val = 0;
    for (element, occurence) in &map {
        if *occurence > val {
            ind = *element;
            val = *occurence;
        }
    }

    println!("The mode is {ind} with {val} occurences");
    
    numbers.sort();
    //Calculate median
    ////Selects the middle (rounded down) element of the sorted list
    println!("The median is {}", numbers[(numbers.len() as i32 / 2) as usize]);
}
