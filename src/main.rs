mod dice;
mod character;
mod common;
mod magic;

use common::{Attribute, DamageLevel};
use common::DamageType::{Physical, Stun};
use character::{Character, Race};
use magic::{Spell};

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

    println!("\n~~ * ~ * ~ * skill stuff * ~ * ~ * ~~");
    let mut jill = Character::new("jill", Race::Ork);
    jill.learn_skill("edged weapons");
    jill.improve_skill_by("edged weapons", 5);
    let roll = jill.skill_test("edged weapons", 4);
    println!("Jill makes a test with edged weapons: {:?}", roll);

    println!("\n~~ * ~ * ~ * spell stuff * ~ * ~ * ~~");
}
