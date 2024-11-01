use criterion::black_box;
use distinct_14::distinct_14_chars::{
    faster_solution, faster_with_array_solution, faster_with_vec_solution, simple_solution,
};
use std::time::Instant;

fn main() {
    let messages = [
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
    ];

    let messages = messages.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();

    check_timings("simple_solution", messages.clone(), simple_solution);

    check_timings("Faster solution", messages.clone(), faster_solution);

    check_timings(
        "Faster solution with Vectors",
        messages.clone(),
        faster_with_vec_solution,
    );

    check_timings(
        "Faster solution with Arrays",
        messages.clone(),
        faster_with_array_solution,
    );

    check_timings(
        "Benny's Solution using Bit Manipulation",
        messages.clone(),
        distinct_14::distinct_14_chars::benny_solution,
    );

    check_timings(
        "David's Solution using Bit Manipulation and rposition",
        messages.clone(),
        distinct_14::distinct_14_chars::david_a_perez_solution,
    );
}

fn check_timings(solution_name: &str, messages: Vec<&[u8]>, function: fn(&[u8]) -> usize) {
    println!();
    println!("{:*^100}", format!(" {} ", solution_name));
    let start_time = Instant::now();
    // println!("Start Time: {:?}", start_time);
    for message in messages {
        for _ in 0..3 {
            black_box(_ = function(black_box(message)))
        }
    }
    let elapsed_time = start_time.elapsed().as_nanos();
    println!("Elapsed time for {}: {:?} ns", solution_name, elapsed_time);
}
