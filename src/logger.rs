use std::vec::Vec;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone)]
struct Item {
    key: String,
    value: String,
}

pub struct Logger {
    art: Vec<String>,
    data: Vec<Item>,
    username: String,
    hostname: String,
    spacing: usize,
}

impl Logger {
    pub fn new() -> Logger {
        let default_art = std::fs::read_to_string("art.txt")
            .expect("no hostname")
            .to_string();
        let lines: Vec<&str> = default_art.trim_end().split('\n').collect();

        let max_length = lines
            .iter()
            .map(|line| line.graphemes(true).count())
            .max()
            .unwrap_or(0);

        let padded_lines: Vec<String> = lines
            .iter()
            .map(|line| {
                let char_count = line.graphemes(true).count();
                let padding = " ".repeat(max_length - char_count);
                format!("{}{}", line, padding)
            })
            .collect();

        Logger {
            art: padded_lines,
            data: Vec::new(),
            username: std::env::var("USER").expect("no user").to_string(),
            hostname: std::fs::read_to_string("/etc/hostname")
                .expect("no hostname")
                .to_string()
                .trim_end()
                .to_string(),
            spacing: 5,
        }
    }

    fn get_fattest_text(&mut self) -> usize {
        let mut fattest = "";
        for count in 0..self.data.len() {
            if self.data[count].key.len() > fattest.len() {
                fattest = &self.data[count].key
            }
        }
        fattest.len()
    }

    pub fn add_item(&mut self, key: &str, value: &str) {
        self.data.push(Item {
            key: key.to_string(),
            value: value.to_string(),
        });
    }

    pub fn output(&mut self) {
        let items = self.data.clone();
        let length = items.len();
        let ascii = self.art.clone();
        let fattest = self.get_fattest_text();
        println!("");
        for count in 0..items.len() + 1 {
            if count < ascii.len() {
                print!(
                    "{}{}{}",
                    std::iter::repeat(" ")
                        .take(self.spacing)
                        .collect::<String>(),
                    ascii[count],
                    std::iter::repeat(" ")
                        .take(self.spacing)
                        .collect::<String>()
                );
            }
            if count == 0 {
                print!("{}@{}", self.username, self.hostname)
            } else {
                let item = items[count - 1].clone();
                let key = item.key;
                let value = item.value;

                if count >= ascii.len() {
                    print!(
                        "{}",
                        std::iter::repeat(" ")
                            .take(ascii.clone()[0].len() + self.spacing + 1)
                            .collect::<String>(),
                    )
                }
                print!(
                    "{}{} -> {}",
                    key,
                    std::iter::repeat(" ")
                        .take(fattest - key.len())
                        .collect::<String>(),
                    value
                )
            }
            print!("\n")
        }

        if self.art.len() > length {
            for i in (length + 1)..self.art.len() {
                print!(
                    "{}{}\n",
                    std::iter::repeat(" ")
                        .take(self.spacing)
                        .collect::<String>(),
                    self.art[i]
                );
            }
        }
    }
}
