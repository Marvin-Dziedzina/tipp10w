#[derive(Debug)]
pub enum Lesson {
    L1,
    L2,
    L3,
    L4,
    L5,
    L6,
    L7,
    L8,
    L9,
    L10,
    L11,
    L12,
    L13,
    L14,
    L15,
    L16,
    L17,
    L18,
    L19,
    L20,
}
impl Lesson {
    pub fn from_lesson_id(n: u8) -> Self {
        match n {
            1 => Lesson::L1,
            2 => Lesson::L2,
            3 => Lesson::L3,
            4 => Lesson::L4,
            5 => Lesson::L5,
            6 => Lesson::L6,
            7 => Lesson::L7,
            8 => Lesson::L8,
            9 => Lesson::L9,
            10 => Lesson::L10,
            11 => Lesson::L11,
            12 => Lesson::L12,
            13 => Lesson::L13,
            14 => Lesson::L14,
            15 => Lesson::L15,
            16 => Lesson::L16,
            17 => Lesson::L17,
            18 => Lesson::L18,
            19 => Lesson::L19,
            20 => Lesson::L20,
            _ => panic!("Could not find lesson!"),
        }
    }

    pub fn get_lesson_name(&self) -> String {
        match self {
            Lesson::L1 => String::from("Lesson 1 (asdf jkl;)"),
            Lesson::L2 => String::from("Lesson 2 (eo)"),
            Lesson::L3 => String::from("Lesson 3 (ti)"),
            Lesson::L4 => String::from("Lesson 4 (nr)"),
            Lesson::L5 => String::from("Lesson 5 (hc)"),
            Lesson::L6 => String::from("Lesson 6 (Capitalization)"),
            Lesson::L7 => String::from("Lesson 7 (wW.,)"),
            Lesson::L8 => String::from("Lesson 8 (gG)"),
            Lesson::L9 => String::from("Lesson 9 (bBuU)"),
            Lesson::L10 => String::from("Lesson 10 (yYmM)"),
            Lesson::L11 => String::from("Lesson 11 (vVpP)"),
            Lesson::L12 => String::from("Lesson 12 (xX?)"),
            Lesson::L13 => String::from("Lesson 13 (qQ!)"),
            Lesson::L14 => String::from("Lesson 14 (zZ()-)"),
            Lesson::L15 => String::from("Lesson 15 (Special Chars 1)"),
            Lesson::L16 => String::from("Lesson 16 (Numerics)"),
            Lesson::L17 => String::from("Lesson 17 (Special Chars 2)"),
            Lesson::L18 => String::from("Lesson 18 (All Characters)"),
            Lesson::L19 => String::from("Lesson 19 (Numpad 1)"),
            Lesson::L20 => String::from("Lesson 20 (Numpad 2)"),
        }
    }

    pub fn get_lesson_id(&self) -> u8 {
        match self {
            Lesson::L1 => 1,
            Lesson::L2 => 2,
            Lesson::L3 => 3,
            Lesson::L4 => 4,
            Lesson::L5 => 5,
            Lesson::L6 => 6,
            Lesson::L7 => 7,
            Lesson::L8 => 8,
            Lesson::L9 => 9,
            Lesson::L10 => 10,
            Lesson::L11 => 11,
            Lesson::L12 => 12,
            Lesson::L13 => 13,
            Lesson::L14 => 14,
            Lesson::L15 => 15,
            Lesson::L16 => 16,
            Lesson::L17 => 17,
            Lesson::L18 => 18,
            Lesson::L19 => 19,
            Lesson::L20 => 20,
        }
    }

    pub fn get_user_lesson(&self) -> u8 {
        self.get_lesson_id() + 100
    }
}
