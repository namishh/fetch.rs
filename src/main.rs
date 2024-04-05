mod info;
mod logger;

fn main() {
    let mut logger = logger::Logger::new(5, String::from("~>"));
    logger.add_item("os", &info::get_os_name());
    logger.add_item("host", &info::get_host());
    logger.add_item("krn", &info::get_kernel_name());
    logger.add_item("wm", &info::get_window_manager());
    logger.add_item("upt", &info::get_uptime());
    logger.output();
}
