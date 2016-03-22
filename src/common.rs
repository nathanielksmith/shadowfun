pub type TargetNumber = i32;

#[derive(Debug)]
pub enum Attribute {
    Willpower,
    Intelligence,
    Body,
    Quickness,
    Strength,
    Charisma,
}

#[derive(Debug)]
pub enum DamageType {
    Stun,
    Physical,
}

#[derive(Debug, Copy, Clone)]
pub enum DamageLevel {
    Light,
    Moderate,
    Serious,
    Deadly,
    Variable,
}

pub trait HasAttrs {
    fn attr(&self, attr:Attribute) -> i32;
}

pub fn dmg_to_num(dlvl: DamageLevel) -> i32 {
    match dlvl {
        DamageLevel::Light => 1,
        DamageLevel::Moderate => 3,
        DamageLevel::Serious => 6,
        // TODO variable
        _ => 0,
    }
}
