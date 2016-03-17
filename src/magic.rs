use common::{Attribute, DamageLevel, SpellTarget, HasAttributes};

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

// TODO this should def extend RollResult
#[derive(Debug)]
pub struct SpellResult {
    pub success: bool,
    pub successes: i32,
    pub drain_result: Option<DamageLevel>,
}

#[derive(Debug)]
pub struct Spell<S: SpellTarget> {
    pub name: &'static str,
    pub force: i32,
    pub drain_level: DamageLevel,
    pub drain_modifier: i32,
    // These are just informational:
    pub spell_type: SpellType,
    pub duration: Duration,
    pub target: S,
}
