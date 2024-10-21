#![feature(iter_array_chunks)]

use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;

use std::io::{BufReader, BufWriter, Read, Write};

use integer_encoding::FixedInt;
use primality_test::IsPrime;

pub fn generate_and_store_primes(path: impl AsRef<Path>, top: u32) -> Result<(), &'static str> {
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .map_err(|_| "Error opening file")?;

    let mut file = BufWriter::new(file);

    for prime in (2u32..).filter(IsPrime::is_prime) {
        file.write(&prime.encode_fixed_light())
            .map_err(|_| "Error writing to file")?;

        if prime > top {
            break;
        }
    }

    Ok(())
}

pub fn load_primes(path: impl AsRef<Path>) -> Result<Vec<u32>, &'static str> {
    let file = File::open(path).map_err(|_| "Error opening file")?;
    let file = BufReader::new(file);

    file.bytes()
        .map(|bytes| bytes.expect("Error reading byte"))
        .array_chunks::<4>()
        .map(|buff| u32::decode_fixed(&buff).ok_or("Error decoding bytes"))
        .collect()
}
