use std::process::Command;

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
    let output = Command::new("uname").arg("-r").output();
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

pub fn get_host() -> String {
    let mut path = "/sys/devices/virtual/dmi/id/product_name";
    if std::fs::metadata(path).is_err() {
        path = "/sys/firmware/devicetree/base/model";
    }

    std::fs::read_to_string(path)
        .expect("no way to get host machine")
        .to_string()
        .trim_end()
        .to_string()
}

pub fn get_window_manager() -> String {
    let xprop_id = Command::new("xprop")
        .args(["-root", "-notype", "_NET_SUPPORTING_WM_CHECK"])
        .output()
        .expect("err");

    let id = String::from_utf8_lossy(&xprop_id.stdout);
    let wm_id = id.split(' ').last().unwrap_or_default();
    let property = Command::new("xprop")
        .args([
            "-id",
            wm_id,
            "-notype",
            "-len",
            "25",
            "-f",
            "_NET_WM_NAME",
            "8t",
        ])
        .output()
        .expect("error");
    let wm = String::from_utf8_lossy(&property.stdout);
    let w = wm.split("\n").collect::<Vec<_>>()[0];
    let w2 = w.split(" ").collect::<Vec<_>>();
    let w3 = w2.last().expect("error");

    w3[1..w3.len() - 1].to_string()
}
