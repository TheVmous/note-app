use anyhow::Result;
use std::fs::File;
use std::path::Path;

pub fn ensure_config() -> Result<()> {
    let path = Path::new("config.toml");
    if path.exists() {
        println!("exist");
        Ok(())
    } else {
        println!("no exist");
        let mut file = File::create("config.toml")?;
        Ok(())
    }
}
