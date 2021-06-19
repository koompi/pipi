pub use std::fs;

pub fn cleaning() -> std::io::Result<()> {
    println!("Cleaning workdir...");

    fs::remove_dir("work/live")?;

    Ok(())
}
