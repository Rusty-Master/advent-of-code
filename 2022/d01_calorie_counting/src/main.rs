use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    // P1
    let input = fs::read_to_string("input.txt")?;

    let mut food_map: HashMap<i32, i32> = HashMap::new();
    let mut counter = 1;

    for line in input.lines() {
        let entry = food_map.entry(counter).or_insert(0);
        if line.is_empty() {
            counter += 1;
        } else {
            let calories: i32 = line.parse().unwrap();
            *entry += calories;
        }
    }

    let highest_value = food_map.values().max().unwrap();

    println!(
        "Elf carrying the most calories is carrying {} calories",
        highest_value
    );

    // P2

    let mut vector: Vec<i32> = food_map.into_values().collect();
    vector.sort();

    let len = vector.len();
    println!(
        "Sum of calories carried by top 3 elves is: {}",
        vector[len - 1] + vector[len - 2] + vector[len - 3]
    );

    Ok(())
}
