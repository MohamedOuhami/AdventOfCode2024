// Read the arrays from the input_data
use std::fs;
pub fn read_data_from_file() -> (Vec<i32>, Vec<i32>) {
    // Creating 2 vectors for the 2 chamber_ids
    let mut loc_id1: Vec<i32> = Vec::new();
    let mut loc_id2: Vec<i32> = Vec::new();
    println!("Reading the file");

    let content = fs::read_to_string("data_inputs/day1_input.txt").expect("Did not find the file");

    let entries = content.trim().split("\n");

    for entry in entries {
        let mut i = 1;
        let real_entries = entry.split("   ");
        for real in real_entries {
            let real: i32 = real.trim().parse().expect("This is not a number");
            if i == 1 {
                loc_id1.push(real);
            } else {
                loc_id2.push(real);
            }
            i += 1;
        }
    }

    let _loc_vecs = (loc_id1, loc_id2);

    _loc_vecs
}

// Return the total distance

pub fn calculate_distance() {
    let (mut loc1, mut loc2) = read_data_from_file();

    let mut distance = 0;

    loc1.sort();
    loc2.sort();

    println!("The first item in loc1 is {}", loc1[1]);
    println!("The first item in loc1 is {}", loc2[1]);
    for i in 0..loc1.len() {
        let difference = loc1[i] - loc2[i];
        distance += difference.abs();
    }

    println!("{}", distance);
}
