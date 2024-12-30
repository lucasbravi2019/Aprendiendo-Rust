use std::collections::HashMap;

fn main() {
    let vector: Vec<i32> = Vec::new();

    let vector_initialized = vec![1, 2, 3];

    let mut mutable_vector: Vec<i32> = Vec::new();

    mutable_vector.push(5);
    mutable_vector.push(6);
    mutable_vector.push(7);
    mutable_vector.push(8);

    let forth_element = mutable_vector.get(3);

    println!("Fourth element using index is: {}", &mutable_vector[3]);
    match forth_element {
        Some(element) => println!("Fourth element is: {}", element),
        None => println!("There is no fourth element"),
    };

    let vector = vec![1, 2, 3, 4];

    for element in &vector {
        println!("{element}");
    }

    let mut vector = vec![1, 2, 3, 4];
    for element in &mut vector {
        *element += 50;
    }

    for element in &vector {
        println!("{element}");
    }

    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("blue")),
    ];

    for element in &row {
        println!("{element:?}");
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    if let Some(element) = scores.get("Blue") { 
        println!("{element}")
    }

    if let Some(element) = scores.get("Red") {
        println!("{element}");
    }

    let blue_team_score = scores.get("Blue").copied().unwrap_or(0);
    println!("Blue team score: {}", blue_team_score);

    for (key, value) in &scores {
        println!("Team name: {}, score: {}", key, value);
    }

    scores.insert(String::from("Red"), 24);

    println!("Red team score: {}", scores.get("Red").copied().unwrap_or(0));

    scores.entry(String::from("Red")).or_insert(50);
    scores.entry(String::from("Green")).or_insert(50);

    let red_team_score_updated = scores.entry(String::from("Red")).or_insert(50);
    *red_team_score_updated += 245;

    println!("Red team score: {}", scores.get("Red").copied().unwrap_or(0));
    println!("Green team score: {}", scores.get("Green").copied().unwrap_or(0));
    println!("{scores:?}")
}
