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
    let oxygenate = Spell {
        name: "oxygenate",
        drain_level: DamageLevel::Light,
        drain_modifier: 2,
        target: 4,
    };

    let confuse = Spell {
        name: "confusion",
        drain_level: DamageLevel::Serious,
        drain_modifier: 0,
        target: Attribute::Willpower,
    };
    jill.learn_spell("confuse");
    jill.learn_skill("sorcery");
    jill.improve_spell_by("confuse", 3);
    jill.improve_skill_by("sorcery", 4);
    jill.learn_spell("oxygenate");
    jill.improve_spell_by("oxygenate", 2);

    let oxy_sr = jill.cast(oxygenate);
    println!("{:?}", oxy_sr);
}
