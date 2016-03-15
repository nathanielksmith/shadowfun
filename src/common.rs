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
