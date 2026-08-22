//! Bounded model-checking harnesses for parser safety and round trips.

use crate::{ParseError, Request, Response};

const MAX_INPUT_LEN: usize = 8;

fn arbitrary_input() -> ([u8; MAX_INPUT_LEN], usize) {
    let bytes = kani::any();
    let len = kani::any();
    kani::assume(len <= MAX_INPUT_LEN);
    (bytes, len)
}

fn assert_parse_invariants<T>(
    input: &[u8],
    before: &[u8; MAX_INPUT_LEN],
    result: Result<(T, usize), ParseError>,
) {
    assert_eq!(input, &before[..input.len()]);

    match result {
        Ok((_, consumed)) => {
            assert!(consumed <= input.len());
            assert!(consumed > 0);
            assert_eq!(input[consumed - 1], b'\n');
        }
        Err(ParseError::Incomplete | ParseError::Invalid) => {
            assert_eq!(input, &before[..input.len()]);
        }
    }
}

#[kani::proof]
#[kani::unwind(10)]
fn response_parse_is_total_and_consumes_through_lf() {
    let (bytes, len) = arbitrary_input();
    let before = bytes;
    let input = &bytes[..len];

    assert_parse_invariants(input, &before, Response::parse(input));
}

#[kani::proof]
#[kani::unwind(10)]
fn request_parse_is_total_and_consumes_through_lf() {
    let (bytes, len) = arbitrary_input();
    let before = bytes;
    let input = &bytes[..len];

    assert_parse_invariants(input, &before, Request::parse(input));
}

#[kani::proof]
fn response_bounded_encode_decode_round_trip() {
    let response = if kani::any() {
        Response::Pong
    } else {
        Response::Error
    };
    let suffix: [u8; 2] = kani::any();
    let mut encoded = [0_u8; MAX_INPUT_LEN];
    let len = response.encode(&mut encoded);
    encoded[len..].copy_from_slice(&suffix[..MAX_INPUT_LEN - len]);

    let (decoded, consumed) = Response::parse(&encoded).unwrap();
    assert_eq!(decoded, response);
    assert_eq!(consumed, len);
}

#[kani::proof]
fn request_bounded_encode_decode_round_trip() {
    let request = Request::Ping;
    let suffix: [u8; 2] = kani::any();
    let mut encoded = [0_u8; MAX_INPUT_LEN];
    let len = request.encode(&mut encoded);
    encoded[len..].copy_from_slice(&suffix);

    let (decoded, consumed) = Request::parse(&encoded).unwrap();
    assert_eq!(decoded, request);
    assert_eq!(consumed, len);
}
