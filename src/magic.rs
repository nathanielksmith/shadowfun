use common::{Attribute, DamageLevel};

#[derive(Debug)]
pub enum SpellType {
    Mana,
    Physical,
}

#[derive(Debug)]
pub enum Duration {
    Instant,
    Sustained,
    Permanent,
}

#[derive(Debug)]
pub struct Spell {
    // TODO these prob won't all be pub if I add a ::new
    pub name: &'static str,
    pub force: i32,
    pub drain_level: DamageLevel,
    pub drain_modifier: i32,
    // These are just informational:
    pub spell_type: SpellType,
    pub target: Attribute,
    pub duration: Duration,
}
