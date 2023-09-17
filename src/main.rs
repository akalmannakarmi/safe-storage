mod manager;
use std::fs;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: cargo run <file_path> <password> [-w]");
        return Ok(());
    }

    let file_path = &args[1];
    let password = &args[2];

    if args.len() > 3 && &args[3] == "-w" {
        let data = fs::read(file_path)?;
        let data = std::str::from_utf8(&data)?;
        let mut new_file_path = String::from(file_path);
        if let Some(last_dot_index) = new_file_path.rfind('.') {
            new_file_path.truncate(last_dot_index);
        }
        new_file_path.push_str(".sfsc");
        manager::write(&new_file_path, data, password)?;
    } else {
        let data = manager::read(file_path, password)?;
        println!("{}", data);
    }

    Ok(())
}
