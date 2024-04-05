pub fn get_os_name() -> String {
    let readings = std::fs::read_to_string("/etc/os-release")
        .expect("no hostname")
        .to_string()
        .trim_end()
        .to_string();
    let line = readings.split("\n").collect::<Vec<_>>()[0]
        .split("=")
        .collect::<Vec<_>>()[1];

    if line.len() < 1 {
        return "Error Os".to_string();
    }

    line[1..line.len() - 1].to_string()
}

pub fn get_kernel_name() -> String {
    let output = std::process::Command::new("uname").arg("-r").output();
    match output {
        Ok(output) => {
            if output.status.success() {
                let version = String::from_utf8_lossy(&output.stdout);
                return version.trim().to_string();
            } else {
                return "Linux Kernel".to_string();
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            return "Kernel Error".to_string();
        }
    }
}

pub fn get_uptime() -> String {
    let mut proc_uptime = std::fs::read_to_string("/proc/uptime")
        .expect("no /proc/uptime")
        .to_string()
        .trim_end()
        .to_string();

    proc_uptime = proc_uptime.split('.').collect::<Vec<&str>>()[0].to_string();
    let seconds = proc_uptime.parse::<u64>().unwrap();
    let days: u64 = seconds / 60 / 60 / 24;
    let hours: u64 = (seconds / 60 / 60) % 24;
    let minutes: u64 = (seconds / 60) % 60;

    let mut slice = format!("{}d {}h {}m", days, hours, minutes);
    if days == 0 {
        slice = format!("{}h {}m", hours, minutes);
    } else if hours == 0 && days == 0 {
        slice = format!("{}m", minutes);
    }

    slice.to_string()
}
