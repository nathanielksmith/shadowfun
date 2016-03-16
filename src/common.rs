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
pub enum DamageLevel {
    Light,
    Moderate,
    Serious,
    Deadly,
    Variable,
}

pub trait HasAttributes {
    fn attr(&self, attribute:&Attribute) -> i32;
}

pub trait SpellTarget {
    fn to_target<T: HasAttributes>(&self, other: &T) -> i32;
}

impl SpellTarget for i32 {
    fn to_target<T: HasAttributes>(&self, _:&T) -> i32 {*self}
}

impl SpellTarget for Attribute {
    fn to_target<T: HasAttributes>(&self, t:&T) -> i32 {
        t.attr(self)
    }
}
