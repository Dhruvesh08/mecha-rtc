use std::process::Command;

fn main() {
    let output = Command::new("timedatectl")
        .arg("status")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let parsed_output = parse_timedatectl_output(&stdout);
        println!("{:#?}", parsed_output);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command failed: {}", stderr);
    }
}

fn parse_timedatectl_output(output: &str) -> TimedateCtlStatus {
    let mut timedatectl_status = TimedateCtlStatus::default();

    for line in output.lines() {
        let mut parts = line.trim().splitn(2, ':');

        if let Some(key) = parts.next() {
            if let Some(value) = parts.next() {
                let key = key.trim();
                let value = value.trim();

                match key {
                    "Local time" => timedatectl_status.local_time = value.to_string(),
                    "Universal time" => timedatectl_status.universal_time = value.to_string(),
                    "RTC time" => timedatectl_status.rtc_time = value.to_string(),
                    "Time zone" => timedatectl_status.time_zone = value.to_string(),
                    "System clock synchronized" => timedatectl_status.system_clock_synchronized = value == "yes",
                    "NTP service" => timedatectl_status.ntp_service = value.to_string(),
                    "RTC in local TZ" => timedatectl_status.rtc_in_local_tz = value == "yes",
                    _ => (),
                }
            }
        }
    }

    timedatectl_status
}

#[derive(Debug, Default)]
struct TimedateCtlStatus {
    local_time: String,
    universal_time: String,
    rtc_time: String,
    time_zone: String,
    system_clock_synchronized: bool,
    ntp_service: String,
    rtc_in_local_tz: bool,
}
