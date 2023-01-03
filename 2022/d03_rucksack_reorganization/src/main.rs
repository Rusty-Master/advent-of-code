use std::{collections::HashMap, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;

    // P.1
    let mut sum = 0;
    let mut rucksack_number = 0;
    let mut duplicates: HashMap<i32, char> = HashMap::new();

    for line in input.lines() {
        rucksack_number += 1;

        let (first, last) = line.split_at(line.len() / 2);

        for c in first.chars() {
            let idx = last.chars().find(|&x| x == c);

            if idx.is_some() && !duplicates.contains_key(&rucksack_number) {
                duplicates.insert(rucksack_number, c);
            }
        }
    }

    let items = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    for duplicate in duplicates.values() {
        sum += items.find(*duplicate).unwrap() + 1;
    }

    println!("Part 2: {:?}", sum);

    //  P.2
    rucksack_number = 0;
    sum = 0;
    let mut group_number = 1;
    let mut groups: HashMap<i32, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        groups
            .entry(group_number)
            .and_modify(|v| v.push(line))
            .or_insert_with(|| vec![line]);

        rucksack_number += 1;
        if rucksack_number % 3 == 0 {
            group_number += 1;
        }
    }

    let mut badges: Vec<char> = vec![];

    for group in groups.values() {
        let badge: Vec<char> = group[0]
            .chars()
            .filter(|c| group[1].contains(*c) && group[2].contains(*c))
            .collect();

        badges.push(badge[0]);
    }

    for badge in badges {
        sum += items.find(badge).unwrap() + 1;
    }

    println!("Part 1: {:?}", sum);

    Ok(())
}
