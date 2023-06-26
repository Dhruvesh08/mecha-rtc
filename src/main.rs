use std::process::Command;

fn get_ntp_time() -> String {
    let output = Command::new("timedatectl")
        .arg("show")
        .arg("--property=NTPSynchronized")
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);

    stdout.to_string()
}

fn get_rtc_time() -> String {
    let output = Command::new("timedatectl")
        .arg("show")
        .arg("--property=LocalRTC")
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);

    stdout.to_string()
}

fn main() {
    println!("NTP time: {}", get_ntp_time());
    println!("RTC time: {}", get_rtc_time());
}
