use dice;

#[derive(Debug)]
enum Race {
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
}

impl Character {
    pub fn reaction(&self) -> i32 {
        (self.intelligence + self.quickness) / 2
    }

    pub fn new() -> Character {
        Character {
            name: "frozboz",
            race: Race::Troll,
            body: 1,
            intelligence: 2,
            strength: 3,
            charisma: 4,
            willpower: 5,
            quickness: 6,
        }
    }
}
