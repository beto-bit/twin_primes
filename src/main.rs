use twin_primes::load_primes;

fn main() {
    twin_primes::generate_and_append_primes("smol.num", 200).unwrap();

    let primes = load_primes("smol.num").unwrap();
    println!("{:?}", primes.last());
}
