mod dice;
mod character;

use character::{Character, Race};
use character::Damage::{Physical, Stun};

fn main() {
    println!("/ / / S H A D O W  F U N \\ \\ \\");

    println!("d6: {}", dice::d6());
    println!("nd6: {}", dice::nd6(4));
    println!("roll: {:?}", dice::roll(12,20));

    let mut froz = Character::new("froz boz", Race::Troll);
    println!("char: {:?}", froz);
    println!("char's reaction: {}", froz.reaction());
    println!("char roll: {:?}", froz.roll(4, 4));
    println!("char after stun: {:?}", froz.injure(Stun, 1));
    println!("char roll: {:?}", froz.roll(4, 4));
    println!("char after phys: {:?}", froz.injure(Physical, 4));
    println!("char roll: {:?}", froz.roll(4, 4));
    println!("char dying: {:?}", froz.injure(Stun, 10));
    println!("char roll: {:?}", froz.roll(4, 4));
    println!("char killed: {:?}", froz.injure(Physical, 7));
    println!("char roll: {:?}", froz.roll(4, 4));
}
