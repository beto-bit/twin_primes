#![feature(iter_array_chunks)]

use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;

use std::io::{BufReader, BufWriter, Read, Write};
use std::io::{Seek, SeekFrom};

use integer_encoding::FixedInt;
use primality_test::IsPrime;

pub fn generate_and_store_primes(path: impl AsRef<Path>, top: u32) -> Result<(), &'static str> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)
        .map_err(|_| "Error opening file")?;

    let mut file = BufWriter::new(file);

    for prime in (2u32..=top).filter(IsPrime::is_prime) {
        file.write(&prime.encode_fixed_light())
            .map_err(|_| "Error writing to file")?;
    }

    Ok(())
}

pub fn generate_and_append_primes(path: impl AsRef<Path>, top: u32) -> Result<(), &'static str> {
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .open(path)
        .map_err(|_| "Error opening file")?;

    // Advance near the end
    file.seek(SeekFrom::End(-4))
        .map_err(|_| "Error seaking near the EOF")?;

    // Read the bytes. It is unbuffered but it doesn't matter since we're doing it only once.
    let mut last_prime_bytes = [0u8; 4];
    file.read(&mut last_prime_bytes)
        .map_err(|_| "Error reading bytes")?;

    let last_prime_found = u32::decode_fixed(&last_prime_bytes).ok_or("Error decoding bytes")?;

    // Store the next primes
    let mut bw = BufWriter::new(file);

    for prime in (last_prime_found + 1..=top).filter(IsPrime::is_prime) {
        bw.write(&prime.encode_fixed_light())
            .map_err(|_| "Error writing to file")?;
    }

    Ok(())
}

pub fn load_primes(path: impl AsRef<Path>) -> Result<Vec<u32>, &'static str> {
    let file = File::open(path).map_err(|_| "Error opening file")?;
    let file = BufReader::new(file);

    file.bytes()
        .map(|byte| byte.expect("Error reading byte"))
        .array_chunks::<4>()
        .map(|buff| u32::decode_fixed(&buff).ok_or("Error decoding bytes"))
        .collect()
}
