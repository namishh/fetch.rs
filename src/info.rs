pub fn get_os_name() -> String {
    let readings = std::fs::read_to_string("/etc/os-release")
        .expect("no hostname")
        .to_string()
        .trim_end()
        .to_string();
    let mut line = readings.split("\n").collect::<Vec<_>>()[0];
    line = line.split("=").collect::<Vec<_>>()[1];

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
