use connect_four::prelude::*;
use std::fs::File;
use std::io::{read_to_string, BufReader};
use std::path::Path;

fn test_eval(test_set_file_name: &str) {
    let path = Path::new("fixtures/");
    let reader = BufReader::new(File::open(path.join(test_set_file_name)).unwrap());
    let test_set_content = read_to_string(reader).expect(format!("failed to read test set {}", test_set_file_name).as_str());
    let test_set = test_set_content
        .lines()
        .map(|line| {
            let position_score = line.split_whitespace().collect::<Vec<&str>>();
            let position = position_score[0];
            let game = Game::new(position);
            let score = position_score[1].parse::<i8>().unwrap();
            (game, score)
        });

    for (i, (mut game, score)) in test_set.enumerate() {
        let eval = game.eval();
        assert_eq!(eval, score, "failed for file: {} on line: {}", test_set_file_name, i + 1);
    }
}

#[test]
fn test_end_easy() { test_eval("Test_L3_R1") }

// #[test]
fn test_middle_easy() { test_eval("Test_L2_R1") }

// #[test]
fn test_middle_medium() { test_eval("Test_L2_R2") }

// #[test]
fn test_begin_easy() { test_eval("Test_L1_R1") }

// #[test]
fn test_begin_medium() { test_eval("Test_L1_R2") }

// #[test]
fn test_begin_hard() { test_eval("Test_L1_R3") }
