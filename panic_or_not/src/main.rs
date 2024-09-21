// use std::net::IpAddr;
fn main() {
    // let home: IpAddr = "127.0.0.1"
    //     .parse()
    //     .expect("Hardcoded IP address should be valid");

    #[derive(Debug)]
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(val: i32) -> Guess {
            if val < 1 || val > 100 {
                panic!("Guess value must be between 1 and 100, got {val}.");
            }

            Guess { value: val }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }

    let guess = Guess::new(33);

    println!(" {:?} is the value of {}", guess, "guess");

    let guess_res = guess.value();
    println!(" {:?} is the value of {}",  guess_res, "guess_res");
}
