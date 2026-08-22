//! Deterministic, stratified parser comparison.
//!
//! This is a repeatable throughput sanity check, not a statistical benchmark.

use ping_proto::{ParseError, Request, Response};
use std::time::{Duration, Instant};

const ITERATIONS_PER_PARSER: usize = 300_000;
const KANI_MAX_INPUT_LEN: usize = 8;

#[derive(Clone, Copy)]
enum Outcome {
    Success = 0,
    Incomplete = 1,
    Invalid = 2,
}

fn outcome<T>(result: Result<(T, usize), ParseError>) -> Outcome {
    match result {
        Ok(_) => Outcome::Success,
        Err(ParseError::Incomplete) => Outcome::Incomplete,
        Err(ParseError::Invalid) => Outcome::Invalid,
    }
}

fn response_input(iteration: usize) -> &'static [u8] {
    const SUCCESS: [&[u8]; 3] = [b"PONG\r\n", b"+OK\r\n", b"-ERR\r\n"];
    const INCOMPLETE: [&[u8]; 3] = [b"", b"PONG\r", b"+WAIT"];
    const INVALID: [&[u8]; 3] = [b"NOPE\r\n", b"XONG\r\n", b"?\n"];

    let stratum = iteration % 3;
    let variant = (iteration / 3) % 3;
    match stratum {
        0 => SUCCESS[variant],
        1 => INCOMPLETE[variant],
        _ => INVALID[variant],
    }
}

fn request_input(iteration: usize) -> &'static [u8] {
    const SUCCESS: [&[u8]; 3] = [b"PING\r\n", b"PING\r\nX", b"PING\r\nPING\r\n"];
    const INCOMPLETE: [&[u8]; 3] = [b"", b"PI", b"PING\r"];
    const INVALID: [&[u8]; 3] = [b"PUNG\r\n", b"ping\r\n", b"X"];

    let stratum = iteration % 3;
    let variant = (iteration / 3) % 3;
    match stratum {
        0 => SUCCESS[variant],
        1 => INCOMPLETE[variant],
        _ => INVALID[variant],
    }
}

fn measure(mut parse: impl FnMut(usize) -> Outcome) -> (Duration, [usize; 3]) {
    let start = Instant::now();
    let mut counts = [0; 3];
    for iteration in 0..ITERATIONS_PER_PARSER {
        counts[parse(iteration) as usize] += 1;
    }
    (start.elapsed(), counts)
}

fn print_result(name: &str, elapsed: Duration, counts: [usize; 3]) {
    let throughput = ITERATIONS_PER_PARSER as f64 / elapsed.as_secs_f64();
    println!(
        "{name}: iterations={} elapsed={elapsed:?} throughput={throughput:.0} parses/s outcomes={{success:{}, incomplete:{}, invalid:{}}}",
        ITERATIONS_PER_PARSER, counts[0], counts[1], counts[2]
    );
    assert!(counts.into_iter().all(|count| count > 0));
}

fn main() {
    let (response_elapsed, response_counts) =
        measure(|iteration| outcome(Response::parse(response_input(iteration))));
    let (request_elapsed, request_counts) =
        measure(|iteration| outcome(Request::parse(request_input(iteration))));

    println!("deterministic stratified parser comparison");
    print_result("response", response_elapsed, response_counts);
    print_result("request", request_elapsed, request_counts);
    println!(
        "Kani raw-domain bound: all byte slices of length 0..={KANI_MAX_INPUT_LEN}; \
         longer inputs and their additional line-length states are outside the proof domain."
    );
}
