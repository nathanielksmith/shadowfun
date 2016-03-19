use std::collections::HashMap;
use std::cmp::max;
use common::{HasAttrs, Attribute};
use dice;
use dice::RollResult;

pub type Skill = &'static str;
pub type SkillLevel = i32;
pub type TargetNumber = i32;

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

    skills: HashMap<Skill, SkillLevel>,

    stun_level: i32,
    phys_level: i32,
}

pub enum Damage {
    Stun,
    Physical,
}

impl Character {
    // TODO consider passing in a Roller. This can then be mocked out in tests.
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

            skills: HashMap::new(),

            phys_level: 0,
            stun_level: 0,
        }
    }

    pub fn learn_skill(&mut self, skill: Skill) -> () {
        self.skills.insert(skill, 1);
    }

    pub fn improve_skill(&mut self, skill: Skill) -> () {
        match self.skills.get_mut(skill) {
            Some(old_level) => *old_level += 1,
            None => (),
        }
    }

    pub fn improve_skill_by(&mut self, skill: Skill, amount: SkillLevel) -> () {
        for _ in 0..amount {
            self.improve_skill(skill)
        };
    }

    pub fn skill(&self, skill: Skill) -> SkillLevel {
        match self.skills.get(skill) {
            Some(level) => *level,
            None => 0,
        }
    }

    pub fn skill_test(&self, skill: Skill, tn: TargetNumber) -> RollResult {
        self.roll(self.skill(skill), tn)
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

    pub fn roll(&self, die: i32, tn: TargetNumber) -> RollResult {
        if self.phys_level > 10 || self.stun_level > 10 {
            println!("WARNING rolling for dead or unconscious character");
        }
        let tn = self.injury_to_mod() + tn;
        return dice::roll(die, tn);
    }
}

impl HasAttrs for Character {
    fn attr(&self, attr:Attribute) -> i32 {
        match attr {
            Attribute::Body => self.body,
            Attribute::Willpower => self.willpower,
            Attribute::Strength => self.strength,
            Attribute::Intelligence => self.intelligence,
            Attribute::Quickness => self.quickness,
            Attribute::Charisma => self.charisma,
        }
    }
}

// tests

#[cfg(test)]
mod tests {
    use character::{Race, Character, Damage};
    use common::{HasAttrs, Attribute};

    #[test]
    fn test_attrs() {
        let mut c = Character::new("flarf", Race::Dwarf);
        c.body = 1;
        c.willpower = 2;
        c.strength = 3;
        c.intelligence = 4;
        c.quickness = 5;
        c.charisma = 6;

        assert_eq!(c.attr(Attribute::Body), 1);
        assert_eq!(c.attr(Attribute::Willpower), 2);
        assert_eq!(c.attr(Attribute::Strength), 3);
        assert_eq!(c.attr(Attribute::Intelligence), 4);
        assert_eq!(c.attr(Attribute::Quickness), 5);
        assert_eq!(c.attr(Attribute::Charisma), 6);
    }

    #[test]
    fn test_skills() {
        let mut c = Character::new("acid", Race::Troll);
        assert_eq!(c.skill("knitting"), 0);
        c.learn_skill("knitting");
        assert_eq!(c.skill("knitting"), 1);
        c.improve_skill("knitting");
        assert_eq!(c.skill("knitting"), 2);
        c.improve_skill_by("knitting", 5);
        assert_eq!(c.skill("knitting"), 7);
        let result = c.skill_test("knitting", 0);
        assert!(result.success);
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

}
