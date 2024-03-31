use std::vec::Vec;

#[derive(Clone)]
struct Item {
    key: String,
    value: String,
}

pub struct Logger {
    art: String,
    data: Vec<Item>,
}

impl Logger {
    pub fn new() -> Logger {
        let default_art = "
 ▄       ▄
▄ ▀▄   ▄▀ ▄
█▄█▀███▀█▄█
▀█████████▀
 ▄▀     ▀▄ 
        "
        .to_string();
        Logger {
            art: default_art,
            data: Vec::new(),
        }
    }

    pub fn get_art(&self) -> &str {
        &self.art
    }

    pub fn add_item(&mut self, key: &str, value: &str) {
        self.data.push(Item {
            key: key.to_string(),
            value: value.to_string(),
        });
    }

    pub fn output(&mut self) {}
}
