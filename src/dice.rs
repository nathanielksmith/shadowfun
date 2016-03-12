extern crate rand;
use self::rand::Rng;

#[derive(Debug)]
pub struct RollResult {
    success: bool,
    catastrophic_fail: bool,
    successes: i32,
}

pub fn d6() -> i32 {
    // TODO use Range
    rand::thread_rng().gen_range(1,7)
}

pub fn nd6(rolls: i32) -> i32 {
    let mut result = 0;
    for _ in 0..rolls {
       result = result + d6();
    }
    result
}

fn explode() -> i32 {
    let mut next_roll = d6();
    let mut result = 6 + next_roll;
    while next_roll == 6 {
        next_roll = d6();
        result += next_roll;
    }
    result
}

pub fn roll(rolls: i32, target: i32) -> RollResult {
    let mut successes = 0;
    let mut ones = 0;
    for _ in 0..rolls {
        let mut result = d6();
        if result == 6 {
            result = explode();
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
