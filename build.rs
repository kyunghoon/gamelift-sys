use anyhow::Result;

fn go() -> Result<()> {
    Ok(())
}

fn main() {
    if let Err(e) = go() {
        println!("cargo:warning={}", e);
    }
}
