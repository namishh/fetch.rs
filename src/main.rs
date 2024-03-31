mod logger;

fn main() {
    let logger = logger::Logger::new();
    let art = logger.get_art();
    println!("{}", art);
    println!("Hello, world!");
}
