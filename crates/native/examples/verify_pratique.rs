fn main() -> Result<(), String> {
    let path = std::path::PathBuf::from(std::env::var_os("LOCALAPPDATA").ok_or("LOCALAPPDATA missing")?).join("Oracle Suite/local-apps.json");
    let app = oracle_suite_native::local_apps::load(&path).into_iter().find(|app| app.id == "pratique").ok_or("Pratique not registered")?;
    app.launch()?;
    println!("Launched {}: {}", app.name, app.executable.display());
    Ok(())
}
