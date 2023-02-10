use std::io;

fn main() -> io::Result<()> {
    let port = freeport::get_free_port()?;
    println!("free port: {}", port);
    Ok(())
}
