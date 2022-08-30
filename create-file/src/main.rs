use std::fs;
use std::io;

fn main() -> std::io::Result<()> {
    println!("Where do you want to create a directory?");
    let mut dir = String::new();
    io::stdin()
        .read_line(&mut dir)
        .expect("Failed to read input");
        
    fs::create_dir("{dir}")?;
    println!("Directory created at {dir}");
    Ok(())
}