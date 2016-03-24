extern crate rand;
use self::rand::Rng;

#[derive(Debug)]
pub struct RollResult {
    pub success: bool,
    pub catastrophic_fail: bool,
    pub successes: i32,
}

pub trait Roller {
    fn new(verbose: bool) -> Self;
    fn verbose(&self) -> bool;

    fn nd6(&self, rolls: i32) -> i32 {
        let mut result = 0;
        for _ in 0..rolls {
            result = result + self.d6();
        }
        result
    }

    fn explode(&self) -> i32 {
        if self.verbose() {
            println!("explosion!");
        }
        let mut next_roll = self.d6();
        let mut result = 6 + next_roll;
        while next_roll == 6 {
            if self.verbose() {
                println!("explosion!");
            }
            next_roll = self.d6();
            result += next_roll;
        }
        result
    }

    fn d6(&self) -> i32 {
        // TODO use Range
        rand::thread_rng().gen_range(1,7)
    }

    fn roll(&self, rolls: i32, target: i32) -> RollResult {
        if self.verbose() {
            println!("Rolling {}d6 with target number {}", rolls, target);
        }
        let mut successes = 0;
        let mut ones = 0;
        for _ in 0..rolls {
            let mut result = self.d6();
            if result == 6 {
                result = self.explode();
            }

            if result == 1 {
                ones += 1;
            }

            if result > target {
                successes += 1;
            }
        }

        RollResult {
            success: successes > 0,
            catastrophic_fail: ones == rolls,
            successes: successes,
        }
    }
}

#[derive(Debug)]
pub struct DefaultRoller {
    verbose: bool,
}

impl Roller for DefaultRoller {
    fn new(verbose: bool) -> Self {
        DefaultRoller {
            verbose: verbose
        }
    }

    fn verbose(&self) -> bool {return self.verbose}
}
