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

// TODO consider a HasAttribute trait for characters / physical objects /
// conjured things

pub trait HasAttributes {
    fn attr(&self, attribute:Attribute) -> i32;
}

pub trait SpellTarget<T:HasAttributes> {
    fn to_target(&self, other: &T) -> i32;
}

impl<T: HasAttributes> SpellTarget<T> for i32 {
    fn to_target(&self, _:&T) -> i32 {*self}
}

impl<T: HasAttributes> SpellTarget<T> for Attribute {
    fn to_target(&self, t:&T) -> i32 {
        t.attr(self)
    }
}
