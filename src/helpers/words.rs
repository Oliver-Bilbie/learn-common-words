use std::io;

use serde_json;

use crate::helpers::random;
use crate::helpers::word_list;

#[derive(Clone, Copy, PartialEq)]
pub enum Languages {
    En,
    De,
    Fr,
    It,
    Es,
}

impl Languages {
    pub fn to_str(&self) -> &str {
        match *self {
            Languages::En => "english",
            Languages::De => "german",
            Languages::Fr => "french",
            Languages::It => "italian",
            Languages::Es => "spanish",
        }
    }

    pub fn from_str(lang: String) -> Languages {
        match lang.as_str() {
            "english" => Languages::En,
            "german" => Languages::De,
            "french" => Languages::Fr,
            "italian" => Languages::It,
            "spanish" => Languages::Es,
            _ => Languages::En,
        }
    }
}

#[derive(Debug)]
pub struct WordOptions {
    pub source: String,
    pub target_correct: String,
    pub target_incorrect_1: String,
    pub target_incorrect_2: String,
}

pub fn get_words(source_lang: Languages, target_lang: Languages, freq_limit: i64) -> WordOptions {
    let random_numbers = random::get_random_integers(freq_limit);

    let word_list = load_words();
    match word_list {
        Ok(word_list) => WordOptions {
            source: word_list[random_numbers[0].to_string()][source_lang.to_str()]
                .to_string()
                .replace("\"", ""),
            target_correct: word_list[random_numbers[0].to_string()][target_lang.to_str()]
                .to_string()
                .replace("\"", ""),
            target_incorrect_1: word_list[random_numbers[1].to_string()][target_lang.to_str()]
                .to_string()
                .replace("\"", ""),
            target_incorrect_2: word_list[random_numbers[2].to_string()][target_lang.to_str()]
                .to_string()
                .replace("\"", ""),
        },
        Err(_) => WordOptions {
            source: "Error".to_string(),
            target_correct: "Error".to_string(),
            target_incorrect_1: "Error".to_string(),
            target_incorrect_2: "Error".to_string(),
        },
    }
}

pub fn load_words() -> Result<serde_json::Value, io::Error> {
    let parsed_words = serde_json::from_str(word_list::WORD_LIST)?;
    match parsed_words {
        serde_json::Value::Object(_) => Ok(parsed_words),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "WORD_LIST is not valid JSON",
        )),
    }
}
