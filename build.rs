use anyhow::Result;

fn go() -> Result<()> {
    unimplemented!("gamelift unsupported on this platform");
}

fn main() {
    if let Err(e) = go() {
        println!("cargo:warning={}", e);
    }
}
