use log::{trace, warn};

/// Enum to represent the different lessons in Tipp10
#[derive(Debug, Clone, PartialEq)]
pub enum LessonSelection {
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
impl LessonSelection {
    /// Get the lesson ID from the lesson number
    pub fn from_lesson_id(n: u8) -> Self {
        trace!("Getting lesson from ID: {}", n);
        match n {
            1 => LessonSelection::L1,
            2 => LessonSelection::L2,
            3 => LessonSelection::L3,
            4 => LessonSelection::L4,
            5 => LessonSelection::L5,
            6 => LessonSelection::L6,
            7 => LessonSelection::L7,
            8 => LessonSelection::L8,
            9 => LessonSelection::L9,
            10 => LessonSelection::L10,
            11 => LessonSelection::L11,
            12 => LessonSelection::L12,
            13 => LessonSelection::L13,
            14 => LessonSelection::L14,
            15 => LessonSelection::L15,
            16 => LessonSelection::L16,
            17 => LessonSelection::L17,
            18 => LessonSelection::L18,
            19 => LessonSelection::L19,
            20 => LessonSelection::L20,
            _ => {
                warn!("Could not find lesson! Defaulting to Lesson 1");
                LessonSelection::L1
            }
        }
    }

    /// Get the lesson number from the lesson name
    pub fn from_lesson_name(name: &str) -> Self {
        match name {
            "Lesson 1 (asdf jkl;)" => LessonSelection::L1,
            "Lesson 2 (eo)" => LessonSelection::L2,
            "Lesson 3 (ti)" => LessonSelection::L3,
            "Lesson 4 (nr)" => LessonSelection::L4,
            "Lesson 5 (hc)" => LessonSelection::L5,
            "Lesson 6 (Capitalization)" => LessonSelection::L6,
            "Lesson 7 (wW.,)" => LessonSelection::L7,
            "Lesson 8 (gG)" => LessonSelection::L8,
            "Lesson 9 (bBuU)" => LessonSelection::L9,
            "Lesson 10 (yYmM)" => LessonSelection::L10,
            "Lesson 11 (vVpP)" => LessonSelection::L11,
            "Lesson 12 (xX?)" => LessonSelection::L12,
            "Lesson 13 (qQ!)" => LessonSelection::L13,
            "Lesson 14 (zZ()-)" => LessonSelection::L14,
            "Lesson 15 (Special Chars 1)" => LessonSelection::L15,
            "Lesson 16 (Numerics)" => LessonSelection::L16,
            "Lesson 17 (Special Chars 2)" => LessonSelection::L17,
            "Lesson 18 (All Characters)" => LessonSelection::L18,
            "Lesson 19 (Numpad 1)" => LessonSelection::L19,
            "Lesson 20 (Numpad 2)" => LessonSelection::L20,
            _ => {
                warn!("Could not find lesson! Defaulting to Lesson 1");
                LessonSelection::L1
            }
        }
    }

    /// Get the lesson name from the Lesson
    pub fn get_lesson_name(&self) -> String {
        match self {
            LessonSelection::L1 => String::from("Lesson 1 (asdf jkl;)"),
            LessonSelection::L2 => String::from("Lesson 2 (eo)"),
            LessonSelection::L3 => String::from("Lesson 3 (ti)"),
            LessonSelection::L4 => String::from("Lesson 4 (nr)"),
            LessonSelection::L5 => String::from("Lesson 5 (hc)"),
            LessonSelection::L6 => String::from("Lesson 6 (Capitalization)"),
            LessonSelection::L7 => String::from("Lesson 7 (wW.,)"),
            LessonSelection::L8 => String::from("Lesson 8 (gG)"),
            LessonSelection::L9 => String::from("Lesson 9 (bBuU)"),
            LessonSelection::L10 => String::from("Lesson 10 (yYmM)"),
            LessonSelection::L11 => String::from("Lesson 11 (vVpP)"),
            LessonSelection::L12 => String::from("Lesson 12 (xX?)"),
            LessonSelection::L13 => String::from("Lesson 13 (qQ!)"),
            LessonSelection::L14 => String::from("Lesson 14 (zZ()-)"),
            LessonSelection::L15 => String::from("Lesson 15 (Special Chars 1)"),
            LessonSelection::L16 => String::from("Lesson 16 (Numerics)"),
            LessonSelection::L17 => String::from("Lesson 17 (Special Chars 2)"),
            LessonSelection::L18 => String::from("Lesson 18 (All Characters)"),
            LessonSelection::L19 => String::from("Lesson 19 (Numpad 1)"),
            LessonSelection::L20 => String::from("Lesson 20 (Numpad 2)"),
        }
    }

    /// Get the lesson ID from the Lesson
    pub fn get_lesson_id(&self) -> u8 {
        match self {
            LessonSelection::L1 => 1,
            LessonSelection::L2 => 2,
            LessonSelection::L3 => 3,
            LessonSelection::L4 => 4,
            LessonSelection::L5 => 5,
            LessonSelection::L6 => 6,
            LessonSelection::L7 => 7,
            LessonSelection::L8 => 8,
            LessonSelection::L9 => 9,
            LessonSelection::L10 => 10,
            LessonSelection::L11 => 11,
            LessonSelection::L12 => 12,
            LessonSelection::L13 => 13,
            LessonSelection::L14 => 14,
            LessonSelection::L15 => 15,
            LessonSelection::L16 => 16,
            LessonSelection::L17 => 17,
            LessonSelection::L18 => 18,
            LessonSelection::L19 => 19,
            LessonSelection::L20 => 20,
        }
    }

    /// Get the user lesson ID from the Lesson
    pub fn get_user_lesson(&self) -> u8 {
        self.get_lesson_id() + 100
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_lesson_id() {
        assert_eq!(LessonSelection::from_lesson_id(1), LessonSelection::L1);
        assert_eq!(LessonSelection::from_lesson_id(20), LessonSelection::L20);
        assert_eq!(LessonSelection::from_lesson_id(0), LessonSelection::L1); // Default case
    }

    #[test]
    fn test_from_lesson_name() {
        assert_eq!(
            LessonSelection::from_lesson_name("Lesson 1 (asdf jkl;)"),
            LessonSelection::L1
        );
        assert_eq!(
            LessonSelection::from_lesson_name("Lesson 20 (Numpad 2)"),
            LessonSelection::L20
        );
        assert_eq!(
            LessonSelection::from_lesson_name("Unknown Lesson"),
            LessonSelection::L1
        ); // Default case
    }

    #[test]
    fn test_get_lesson_name() {
        assert_eq!(
            LessonSelection::L1.get_lesson_name(),
            "Lesson 1 (asdf jkl;)".to_string()
        );
        assert_eq!(
            LessonSelection::L20.get_lesson_name(),
            "Lesson 20 (Numpad 2)".to_string()
        );
    }

    #[test]
    fn test_get_lesson_id() {
        assert_eq!(LessonSelection::L1.get_lesson_id(), 1);
        assert_eq!(LessonSelection::L20.get_lesson_id(), 20);
    }

    #[test]
    fn test_get_user_lesson() {
        assert_eq!(LessonSelection::L1.get_user_lesson(), 101);
        assert_eq!(LessonSelection::L20.get_user_lesson(), 120);
    }
}
