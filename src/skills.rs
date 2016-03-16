#[derive(Debug)]
pub struct Skill {
    pub name: &'static str,
    pub level: i32,
}

impl Skill {
    pub fn new(name: &'static str) -> Skill{
        Skill {
            name: name,
            level: 1,
        }
    }

    pub fn improve(&mut self) -> i32 {
        self.level += 1;
        self.level
    }

    pub fn improve_by(&mut self, level: i32) -> i32 {
        self.level += level;
        self.level
    }
}

pub trait HasSkills {
    fn skill(&self, name:&'static str) -> Option<&Skill>;
    fn learn_skill(&mut self, skill:Skill) -> &mut Self;
}
