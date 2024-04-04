mod info;
mod logger;

fn main() {
    let mut logger = logger::Logger::new();
    logger.add_item("os", &info::get_os_name());
    logger.add_item("krn", &info::get_kernel_name());
    logger.add_item("wm", "dwm");
    logger.output();
}
