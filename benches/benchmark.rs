use connect_four::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs::File;
use std::io::{read_to_string, BufReader};
use std::path::Path;

fn bench_eval(c: &mut Criterion) {
    let path = Path::new("fixtures/");
    for (test_set_name, test_set_file_name) in [
        ("End-Easy", "Test_L3_R1"),
        // ("Middle-Easy", "Test_L2_R1"),
        // ("Middle-Medium", "Test_L2_R2"),
        // ("Begin-Easy", "Test_L1_R1"),
        // ("Begin-Medium", "Test_L1_R2"),
        // ("Begin-Hard", "Test_L1_R3"),
    ] {
        let reader = BufReader::new(File::open(path.join(test_set_file_name)).unwrap());
        let test_set_content = read_to_string(reader).expect(format!("failed to read test set {}", test_set_file_name).as_str());
        let test_set = black_box(test_set_content
            .lines()
            .map(|line| {
                let position_score = line.split_whitespace().collect::<Vec<&str>>();
                let position = position_score[0];
                let game = Game::new(position);
                game
            })
            .collect::<Vec<Game>>()
        );

        c.bench_function(test_set_name, |b| {
            b.iter_batched(
                || test_set.iter().clone(),
                |test_set| for game in test_set { game.eval(); },
                criterion::BatchSize::LargeInput,
            );
        });
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(10);
    targets = bench_eval
}
criterion_main!(benches);
