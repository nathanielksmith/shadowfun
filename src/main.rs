mod dice;
mod character;
mod common;
mod magic;

use common::{Attribute, DamageLevel};
use common::DamageType::{Physical, Stun};
use dice::{DefaultRoller, Roller};
use character::{Character, Race};
use magic::{Spell};

fn main() {
    println!("/ / / S H A D O W  F U N \\ \\ \\");

    let roller = DefaultRoller::new(true);

    println!("d6: {}", roller.d6());
    println!("nd6: {}", roller.nd6(4));
    println!("roll: {:?}", roller.roll(12,20));

    let mut froz = Character::new("froz boz", Race::Troll, &roller);
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
    let mut jill = Character::new("jill", Race::Ork, &roller);
    jill.learn_skill("edged weapons");
    jill.improve_skill_by("edged weapons", 5);
    let roll = jill.skill_test("edged weapons", 4);
    println!("Jill makes a test with edged weapons: {:?}", roll);

    println!("\n~~ * ~ * ~ * spell stuff * ~ * ~ * ~~");
    let oxygenate = Spell {
        name: "oxygenate",
        drain_level: DamageLevel::Light,
        drain_modifier: 2,
        target: 4,
    };

    let confuse = Spell {
        name: "confuse",
        drain_level: DamageLevel::Serious,
        drain_modifier: 0,
        target: Attribute::Willpower,
    };
    jill.willpower = 3;
    jill.learn_spell("confuse");
    jill.learn_skill("sorcery");
    jill.improve_spell_by("confuse", 5);
    jill.improve_skill_by("sorcery", 4);
    jill.learn_spell("oxygenate");
    jill.improve_spell_by("oxygenate", 2);

    println!("jill is casting oxygenate");
    let oxy_sr = jill.cast(&oxygenate);
    println!("\t{:?}", oxy_sr);

    let mut frank = Character::new("frank", Race::Elf, &roller);
    frank.willpower = 4;
    println!("jill is casting confuse at frank");
    let conf_sr = jill.cast_at(&confuse, &frank);
    if conf_sr.success {
        println!("jill succeeded at confusing frank");
        if let Some(dl) = conf_sr.drain_result {
            println!("jill suffered some {:?} drain from her spell :(", dl);
        }
    } else {
        println!("frank resisted jill's confuse spell");
    }
}
