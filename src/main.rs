use std::fmt::Display;
use std::fmt::Result;
use std::fmt::Formatter;

enum FzBzV {
    Value(u64),
    Fizz(u64),
    Buzz(u64),
    FizzBuzz(u64),
}

impl Display for FzBzV {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            &FzBzV::Value(n) => write!(f, "{}", n),
            &FzBzV::Fizz(_) => write!(f, "Fizz"),
            &FzBzV::Buzz(_) => write!(f, "Buzz"),
            &FzBzV::FizzBuzz(_) => write!(f, "FizzBuzz"),
        }
    }
}

fn main() {
    for n in (1..101).map(to_fizzbuzz) {
        println!("{}", n);
    }
}

fn to_fizzbuzz(n: u64) -> FzBzV {
    match n {
        n if n % 15 == 0 => FzBzV::FizzBuzz(n),
        n if n % 5 == 0 => FzBzV::Buzz(n),
        n if n % 3 == 0 => FzBzV::Fizz(n),
        n => FzBzV::Value(n),
    }
}