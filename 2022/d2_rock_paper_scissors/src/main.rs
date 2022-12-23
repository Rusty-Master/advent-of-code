use std::{fs, error::Error, collections::HashMap};

fn main() -> Result<(), Box<dyn Error>>{
    //P1

    /*
    Row 1: 
    A - Rock
    B - Paper
    C - Scissors 
    
    Row 2: 
    X - Rock
    Y - Paper
    Z - Scissors 
    */
    let input = fs::read_to_string("input.txt")?;

    let mut total_score = 0;
    let mut selection_score_map = HashMap::new();
    selection_score_map.insert("X", 1);
    selection_score_map.insert("Y", 2);
    selection_score_map.insert("Z", 3);

    for line in input.lines() {

        let round_points = selection_score_map.get(line.get(2..3).unwrap()).unwrap();
        let opponents_choice = line.get(..1).unwrap();
        let my_choice = line.get(2..3).unwrap();
        
        let fight_points = match (opponents_choice, my_choice) {
            ("A","X") => 3,
            ("A","Y") => 6,
            ("A","Z") => 0,
            ("B","X") => 0,
            ("B","Y") => 3,
            ("B","Z") => 6,
            ("C","X") => 6,
            ("C","Y") => 0,
            ("C","Z") => 3,
            (_,_) => panic!(),
        };
        total_score = total_score + fight_points + round_points;
    }
    println!("My total score: {}",total_score);
    
    //P2
    /*
        X - lose
        Y - draw
        Z - win
    */

    let mut total_score = 0;
    let mut selection_score_map = HashMap::new();
    selection_score_map.insert("A", 1);
    selection_score_map.insert("B", 2);
    selection_score_map.insert("C", 3);

    for line in input.lines() {

        
        let opponents_choice = line.get(..1).unwrap();
        let strategy = line.get(2..3).unwrap();
        
        let my_choice = match (opponents_choice, strategy) {
            ("A","X") => "C",
            ("A","Y") => "A",
            ("A","Z") => "B",
            ("B","X") => "A",
            ("B","Y") => "B",
            ("B","Z") => "C",
            ("C","X") => "B",
            ("C","Y") => "C",
            ("C","Z") => "A",
            (_,_) => panic!(),
        };

        let round_points = selection_score_map.get(my_choice).unwrap();
        
        let fight_points = match strategy {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => panic!(),
        };

        total_score = total_score + fight_points + round_points;
    }
    println!("My total score: {}",total_score);

    Ok(())
}
