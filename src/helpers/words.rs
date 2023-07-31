use serde_json;

use crate::helpers::random;
use crate::helpers::word_list;

#[derive(Clone, Copy, PartialEq)]
pub enum Languages {
    En,
    De,
}

impl Languages {
    pub fn to_str(&self) -> &str {
        match *self {
            Languages::En => "english",
            Languages::De => "german",
        }
    }

    pub fn from_str(lang: String) -> Languages {
        match lang.as_str() {
            "english" => Languages::En,
            "german" => Languages::De,
            _ => Languages::En,
        }
    }
}

#[derive(Debug)]
pub struct WordOptions {
    pub native: String,
    pub foreign_correct: String,
    pub foreign_incorrect_1: String,
    pub foreign_incorrect_2: String,
}

pub fn get_words(native_lang: Languages, foreign_lang: Languages, freq_limit: i64) -> WordOptions {
    let random_numbers = random::get_random_integers(freq_limit);

    let word_list: serde_json::Value = load_words();

    WordOptions {
        native: word_list[random_numbers[0].to_string()][native_lang.to_str()]
            .to_string()
            .replace("\"", ""),
        foreign_correct: word_list[random_numbers[0].to_string()][foreign_lang.to_str()]
            .to_string()
            .replace("\"", ""),
        foreign_incorrect_1: word_list[random_numbers[1].to_string()][foreign_lang.to_str()]
            .to_string()
            .replace("\"", ""),
        foreign_incorrect_2: word_list[random_numbers[2].to_string()][foreign_lang.to_str()]
            .to_string()
            .replace("\"", ""),
    }
}

pub fn load_words() -> serde_json::Value {
    serde_json::from_str(word_list::WORD_LIST).expect("Failed to parse JSON")
}
