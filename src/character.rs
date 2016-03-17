use std::cmp::max;
use std::ascii::AsciiExt;
use dice;
use dice::RollResult;
use common::{Attribute, HasAttributes, SpellTarget, dmg_to_num};
use magic::{Spell, SpellResult};
use skills::{Skill, HasSkills};

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
    pub skills: Vec<Skill>,
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
            body: 1,
            intelligence: 2,
            strength: 3,
            charisma: 4,
            willpower: 5,
            quickness: 6,
            magic: 6,

            phys_level: 0,
            stun_level: 0,
            skills: vec![],
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

    pub fn cast<S: SpellTarget> (&mut self, spell: Spell<S>) -> SpellResult
    {
        // TODO casting at less than max force
        if let None = self.skill("sorcery") {
            return SpellResult {
                success: false,
                successes: 0,
                drain_result: None,
            }
        }
        let tn = spell.target.to_target(self);
        let num_die = self.skill("sorcery").unwrap().level;
        let cast_roll = self.roll(num_die, tn);
        if !cast_roll.success {
            return SpellResult {
                success: false,
                successes: 0,
                drain_result: None,
            }
        }

        let num_die = self.attr(&Attribute::Willpower);
        // not using self.roll since we ignore target modifiers for drain
        let drain_roll = dice::roll(num_die, spell.force / 2);
        if drain_roll.success {
            return SpellResult {
                success: true,
                successes: cast_roll.successes,
                drain_result: None,
            }
        }

        // TODO lessen damage by a level per 2 successes
        let damage_type = if spell.force > self.magic {
            Damage::Physical
        } else {
            Damage::Stun
        };
        self.injure(damage_type, dmg_to_num(spell.drain_level));
        SpellResult {
            success: true,
            successes: cast_roll.successes,
            drain_result: Some(spell.drain_level),
        }
    }
}

impl HasAttributes for Character {
     fn attr(&self, attribute:&Attribute) -> i32 {
        match attribute {
            &Attribute::Willpower => self.willpower,
            &Attribute::Intelligence => self.intelligence,
            &Attribute::Body => self.body,
            &Attribute::Quickness => self.quickness,
            &Attribute::Strength => self.strength,
            &Attribute::Charisma => self.charisma,
        }
    }
}

impl HasSkills for Character {
    fn skill(&mut self, name:&'static str) -> Option<&Skill> {
        // TODO case insensitive
        for skill in &self.skills {
            if name.eq_ignore_ascii_case(skill.name) {
                return Some(skill)
            }
        }
        None
    }

    fn learn_skill(&mut self, skill:Skill) -> () {
        if let None = self.skill(skill.name) {
            self.skills.push(skill);
        }
    }
}

// tests

#[test]
fn test_attrs() {
    let mut c = Character::new("herb", Race::Elf);
    c.quickness = 1;
    c.body = 2;
    c.willpower = 3;
    c.intelligence = 4;
    c.strength = 5;
    c.charisma = 6;
    assert_eq!(c.attr(&Attribute::Quickness), 1);
    assert_eq!(c.attr(&Attribute::Body), 2);
    assert_eq!(c.attr(&Attribute::Willpower), 3);
    assert_eq!(c.attr(&Attribute::Intelligence), 4);
    assert_eq!(c.attr(&Attribute::Strength), 5);
    assert_eq!(c.attr(&Attribute::Charisma), 6);
}

#[test]
fn test_skills() {
    let mut c = Character::new("sara", Race::Ork);
    assert_eq!(c.skills.len(), 0);
    c.learn_skill(Skill::new("knitting"));
    {
        let s = c.skill("knitting").unwrap();
        assert_eq!(s.name, "knitting");
        assert_eq!(s.level, 1);
    }
    assert_eq!(c.skills.len(), 1);

    // ensure that a skill is learned only once:
    c.learn_skill(Skill::new("knitting"));
    assert_eq!(c.skills.len(), 1);

    // ensure that skill lookup is not case sensitive
    assert!(match c.skill("Knitting") {
        Some(_) => true,
        _ => false
    });

    assert!(match c.skill("nope") {
        None => true,
        _ => false
    })
}

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
