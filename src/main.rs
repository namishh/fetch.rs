mod logger;

fn main() {
    let mut logger = logger::Logger::new();
    logger.add_item("os", "arch");
    logger.add_item("krn", "something something");
    logger.add_item("wm", "dwm");
    logger.output();
}
