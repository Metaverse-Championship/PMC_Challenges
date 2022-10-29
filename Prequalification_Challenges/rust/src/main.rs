use std::fs;
extern crate num_bigint;
extern crate rand;
extern crate byte_string;
extern crate num_primes;
use num_bigint::{BigInt};
//use num_primes::Generator;

extern crate num;
use num::Integer;
use num::One;
use num::Zero;
use std::convert::TryInto;

fn pow(n: BigInt, exp: BigInt) -> BigInt {
    n.pow(exp.try_into().expect("exponent too large for pow()"))
}

fn modinv(n: &BigInt, p: &BigInt) -> BigInt {
    if p.is_one() { return BigInt::one() }

    let (mut a, mut m, mut x, mut inv) = (n.clone(), p.clone(), BigInt::zero(), BigInt::one());

    while a > BigInt::one() {
        let (div, rem) = a.div_rem(&m);
        inv -= div * &x;
        a = rem;
        std::mem::swap(&mut a, &mut m);
        std::mem::swap(&mut x, &mut inv);
    }
 
    if inv < BigInt::zero() { inv += p }

    inv
}
fn lcm(first: BigInt, second: BigInt) -> BigInt {
    &first * &second / gcd(first, second)
}

fn gcd(first: BigInt, second: BigInt) -> BigInt {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = &max % &min;
        if res ==  BigInt::parse_bytes(b"0", 2).unwrap(){
            return min;
        }

        max = min;
        min = res;
    }
}

fn main() -> std::io::Result<()>{
    let contents = fs::read_to_string("./flag.txt")
        .expect("We lost the flag!");
    let contents = contents.to_string();
    let contents = contents.trim().to_string();
    let mut binary_flag = "".to_string();
    for character in contents.clone().bytes() {
        binary_flag += &format!("{0:08b}", character);
    }
    let flag_int = BigInt::parse_bytes(binary_flag.as_bytes(), 2).unwrap();
    // let p: BigInt = rng.sample(RandomBits::new(512));
    // let q: BigInt = rng.sample(RandomBits::new(512));
    let p_ = fs::read_to_string("./p.txt")
        .expect("We lost the p number! You can create p.txt with a large prime P");
    let p_ = p_.to_string();
    let p_ = p_.trim().to_string();
    let p = BigInt::parse_bytes(p_.as_bytes(), 10).unwrap();
    
    let q_ = fs::read_to_string("./q.txt")
        .expect("We lost the q number! You can create q.txt with a large prime Q");
    let q_ = q_.to_string();
    let q_ = q_.trim().to_string();
    let q = BigInt::parse_bytes(q_.as_bytes(), 10).unwrap();
    
    let n: BigInt = &p * &q;
    println!("n = {}", &n);
    let one: BigInt =  BigInt::parse_bytes(b"1", 2).unwrap();
    let one_: BigInt =  BigInt::parse_bytes(b"1", 2).unwrap();

    let e: BigInt = BigInt::parse_bytes(b"65537", 10).unwrap();
    let phi = lcm(&p - &one, &q - &one);
    let d: BigInt = modinv(&e, &phi);
    let n_1: BigInt = &n + one; 
    let eight: BigInt = BigInt::parse_bytes(b"1000", 2).unwrap();
    let pow_n_2: BigInt = &n * &n; 
    let d_8: BigInt = &d /&eight; 
    let c1: BigInt = n_1.modpow(&d_8, &pow_n_2);
    let c2: BigInt = flag_int.modpow(&e, &n);
    println!("c1 = {}", &c1);
    println!("c2 = {}", &c2);

    Ok(())
}
