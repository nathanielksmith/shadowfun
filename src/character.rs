use std::cmp::max;
use dice;
use dice::RollResult;
use common::{Attribute, HasAttributes, SpellTarget};
use magic::{Spell, SpellResult};

#[derive(Debug)]
pub enum Race {
    Human,
    Ork,
    Elf,
    Dwarf,
    Troll,
}

#[derive(Debug)]
pub struct Character {
    name: &'static str,
    body: i32,
    race: Race,
    intelligence: i32,
    strength: i32,
    charisma: i32,
    willpower: i32,
    quickness: i32,
    // TODO essence
    magic: i32,

    stun_level: i32,
    phys_level: i32,
}

pub enum Damage {
    Stun,
    Physical,
}

impl Character {
    pub fn new(name: &'static str, race: Race) -> Character {
        Character {
            name: name,
            race: race,
            body: 0,
            intelligence: 0,
            strength: 0,
            charisma: 0,
            willpower: 0,
            quickness: 0,
            magic: 6,

            phys_level: 0,
            stun_level: 0,
        }
    }

    pub fn reaction(&self) -> i32 {
        (self.intelligence + self.quickness) / 2
    }

    pub fn injure(&mut self, kind: Damage, amount: i32) -> &Self {
        match kind {
            Damage::Stun => {
                if self.stun_level + amount >= 10 {
                    self.phys_level += amount - (10 - self.stun_level);
                    self.stun_level = 10;
                    println!("WARNING: {} has fallen unconscious.", self.name);
                } else {
                    self.stun_level += amount;
                }
            },
            Damage::Physical => {
                self.phys_level += amount;
                if self.phys_level > 10 {
                    println!("WARNING: {} has died.", self.name);
                }
            }
        };
        self
    }

    fn injury_to_mod(&self) -> i32 {
        match max(self.stun_level, self.phys_level) {
            0 => 0,
            1...2 => 1,
            3...5 => 2,
            _ => 3,
        }
    }

    pub fn roll(&self, die: i32, tn: i32) -> RollResult {
        if self.phys_level > 10 || self.stun_level > 10 {
            println!("WARNING rolling for dead or unconscious character");
        }
        let tn = self.injury_to_mod() + tn;
        return dice::roll(die, tn);
    }

    //fn spell_test<T: HasAttributes, S: SpellTarget<T>>
    //    (&self, spell: Spell<T, S>)
    //     -> RollResult
    //{
    //    // TODO basic fizzle test
    //    RollResult {
    //        success: false,
    //        catastrophic_fail: false,
    //        successes: 0,
    //    }
    //}

    //fn drain_test<T: HasAttributes, S: SpellTarget<T>>
    //    (&self, spell: Spell<T,S>) -> RollResult
    //{
    //    // TODO calucate drain, apply damage
    //    RollResult {
    //        success: false,
    //        catastrophic_fail: false,
    //        successes: 0,
    //    }
    //}

    //pub fn cast_at<T: HasAttributes, S: SpellTarget<T>>
    //    (&self, spell: Spell<T, S>, target: Character) -> SpellResult
    //{
    //    // TODO Fizzle test
    //    // TODO Resistance test
    //    // TODO Drain test
    //    SpellResult {
    //        success: false,
    //    }
    //}

    //pub fn cast<T: HasAttributes, S: SpellTarget<T>>
    //    (&self, spell: Spell<T, S>) -> SpellResult
    //{
    //    // TODO Fizzle test
    //    // TODO Drain test
    //    SpellResult {
    //        success: false,
    //    }
    //}
}

impl HasAttributes for Character {
    pub fn attr(&self, attribute:Attribute) -> i32 {
        match attribute {
            Willpower => self.willpower,
            Intelligence => self.intelligence,
            Body => self.body,
            Quickness => self.quickness,
            Strength => self.strength,
            Charisma => self.charisma,
        }
    }

}

// tests

#[test]
fn test_reaction() {
    let mut c = Character::new("juli", Race::Human);
    c.quickness = 3;
    c.intelligence = 1;
    assert_eq!(c.reaction(), 2);
    c.intelligence = 4;
    assert_eq!(c.reaction(), 3)
}

#[test]
fn test_condition() {
    let mut c = Character::new("hernando", Race::Elf);
    assert_eq!(c.phys_level, 0);
    assert_eq!(c.stun_level, 0);
    c.injure(Damage::Stun, 1);
    assert_eq!(c.phys_level, 0);
    assert_eq!(c.stun_level, 1);
    c.injure(Damage::Physical, 1);
    assert_eq!(c.phys_level, 1);
    assert_eq!(c.stun_level, 1);
    c.injure(Damage::Stun, 11);
    assert_eq!(c.phys_level, 3);
    assert_eq!(c.stun_level, 10);
}

#[test]
fn test_injury_mod() {
    let mut c = Character::new("francine", Race::Dwarf);
    c.injure(Damage::Stun, 1);
    assert_eq!(c.injury_to_mod(), 1);
    c.injure(Damage::Physical, 3);
    assert_eq!(c.injury_to_mod(), 2);
    c.injure(Damage::Stun, 6);
    assert_eq!(c.injury_to_mod(), 3);
}
