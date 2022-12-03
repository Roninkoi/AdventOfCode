use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;

#[derive(PartialEq, Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(PartialEq, Clone, Copy)]
enum Score {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

fn parse_shape(input: &str) -> Option<Shape> {
    let mut shape = None;
    match input {
        "A" | "X" => {
            // rock
            shape = Some(Shape::Rock);
        }
        "B" | "Y" => {
            // paper
            shape = Some(Shape::Paper);
        }
        "C" | "Z" => {
            // scissors
            shape = Some(Shape::Scissors);
        }
        _ => {}
    }
    return shape;
}

fn parse_score(input: &str) -> Option<Score> {
    let mut score = None;
    match input {
        "X" => {
            // lose
            score = Some(Score::Loss);
        }
        "Y" => {
            // draw
            score = Some(Score::Draw);
        }
        "Z" => {
            // win
            score = Some(Score::Win);
        }
        _ => {}
    }
    return score;
}

fn get_score(opp: Shape, me: Shape) -> Option<Score> {
    let mut result = None;

    if me == opp {
        result = Some(Score::Draw);
    }
    match opp {
        Shape::Rock => {
            if me == Shape::Paper {
                result = Some(Score::Win);
            } else if me == Shape::Scissors {
                result = Some(Score::Loss);
            }
        }
        Shape::Paper => {
            if me == Shape::Rock {
                result = Some(Score::Loss);
            } else if me == Shape::Scissors {
                result = Some(Score::Win);
            }
        }
        Shape::Scissors => {
            if me == Shape::Rock {
                result = Some(Score::Win);
            } else if me == Shape::Paper {
                result = Some(Score::Loss);
            }
        }
        _ => {}
    }

    return result;
}

fn pick_shape(opp: Shape, end_score: Score) -> Option<Shape> {
    let mut shape = None;

    match end_score {
        Score::Win => {
            if opp == Shape::Rock {
                shape = Some(Shape::Paper);
            } else if opp == Shape::Paper {
                shape = Some(Shape::Scissors);
            } else if opp == Shape::Scissors {
                shape = Some(Shape::Rock);
            }
        }
        Score::Draw => {
            shape = Some(opp);
        }
        Score::Loss => {
            if opp == Shape::Rock {
                shape = Some(Shape::Scissors);
            } else if opp == Shape::Paper {
                shape = Some(Shape::Rock);
            } else if opp == Shape::Scissors {
                shape = Some(Shape::Paper);
            }
        }
        _ => {}
    }

    return shape;
}

pub fn day2_1(file_path: &str) {
    let stdin = io::stdin();
    let file = File::open(file_path).expect("File not found!");
    let reader = BufReader::new(file);

    let mut score = 0;
    for line in reader.lines() {
        let l = line.unwrap();
        let words = l.split_whitespace().collect::<Vec<&str>>();
        if words.len() < 2 {
            continue;
        }
        let opp = parse_shape(words[0]).unwrap();
        let me = parse_shape(words[1]).unwrap();

        let mut result = get_score(opp, me).unwrap();
        println!("Opponent: {} me: {} result: {}", opp as i32, me as i32, result as i32);

        score += result as i32 + me as i32;
    }

    println!("Total score: {score}");
}

pub fn day2_2(file_path: &str) -> io::Result<()> {
    let stdin = io::stdin();
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut score = 0;
    for line in reader.lines() {
        let l = line?;
        let words = l.split_whitespace().collect::<Vec<&str>>();
        if words.len() < 2 {
            continue;
        }
        let opp = parse_shape(words[0]).unwrap();
        let end_score = parse_score(words[1]).unwrap();

        let me = pick_shape(opp, end_score).unwrap();

        let mut result = get_score(opp, me).unwrap();
        println!("Opponent: {} me: {} result: {}", opp as i32, me as i32, result as i32);

        score += result as i32 + me as i32;
    }

    println!("Total score: {score}");

    Ok(())
}
