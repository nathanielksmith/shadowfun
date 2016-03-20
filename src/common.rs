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

#[derive(Debug)]
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
