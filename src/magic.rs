use common::{Attribute, DamageLevel, TargetNumber, HasAttrs};
use dice::RollResult;

pub type SpellName = &'static str;
pub type ForceLevel = i32;

pub trait SpellTargetNumber {
    fn to_tn<T:HasAttrs>(&self, spell_target: &T) -> TargetNumber;
}

impl SpellTargetNumber for Attribute {
    fn to_tn<T:HasAttrs>(&self, spell_target:&T) -> TargetNumber {
        // TODO this seems dumb.
        match self {
            &Attribute::Willpower => spell_target.attr(Attribute::Willpower),
            &Attribute::Strength => spell_target.attr(Attribute::Strength),
            &Attribute::Intelligence => spell_target.attr(Attribute::Intelligence),
            &Attribute::Charisma => spell_target.attr(Attribute::Charisma),
            &Attribute::Quickness => spell_target.attr(Attribute::Quickness),
            &Attribute::Body => spell_target.attr(Attribute::Body),
        }
    }
}

impl SpellTargetNumber for i32 {
    fn to_tn<T:HasAttrs>(&self, _: &T) -> TargetNumber {
        *self
    }
}

#[derive(Debug)]
pub struct Spell<T: SpellTargetNumber> {
    pub name: &'static str,
    pub drain_level: DamageLevel,
    pub drain_modifier: i32,
    pub target: T
}

impl<S> Spell<S> where S: SpellTargetNumber {
    pub fn to_tn<T:HasAttrs>(&self, target: &T) -> TargetNumber {
        self.target.to_tn(target)
    }
}

#[derive(Debug)]
pub struct SpellResult {
    pub success: bool,
    pub successes: i32,
    pub drain_result: Option<DamageLevel>,
}

impl SpellResult {
    pub fn from_roll
        (roll: RollResult, drain_damage: Option<DamageLevel>)
         -> Self
    {
        SpellResult {
            success: roll.success,
            successes: roll.successes,
            drain_result: drain_damage,
        }
    }
}
