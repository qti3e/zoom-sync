mod info;
mod weather;
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

    let (icon, min, max, temp) = weather::get_weather(None).await;
    keyboard
        .set_weather(icon, temp as u8, max as u8, min as u8)
        .map_err(|e| format!("failed to set weather: {e}"))?;
    println!("updated weather {{ current: {temp}, min: {min}, max: {max} }}");

    let gpu = info::GpuTemp::new(None);

    keyboard
        .set_system_info(42, gpu.get_temp().unwrap_or_default(), 0.)
        .map_err(|e| format!("failed to set system info: {e}"))?;
    println!("updated system info");

    Ok(())
}
