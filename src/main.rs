mod zoom65;

#[tokio::main]
async fn main() -> Result<(), String> {
    let mut keyboard =
        zoom65::Zoom65v3::open().map_err(|e| format!("failed to open device: {e}"))?;

    let version = keyboard
        .get_version()
        .map_err(|e| format!("failed to get keyboard version: {e}"))?;
    println!("keyboard version: {version}",);

    let time = chrono::Local::now();
    keyboard
        .set_time(time)
        .map_err(|e| format!("failed to set time: {e}"))?;
    println!("updated time to {time}");

    keyboard
        .set_weather(0, 1, 2, 3)
        .map_err(|e| format!("failed to set weather: {e}"))?;
    println!("updated weather");

    keyboard
        .set_system_info(42, 69, 0.)
        .map_err(|e| format!("failed to set system info: {e}"))?;
    println!("updated system info");

    Ok(())
}
