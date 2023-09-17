mod manager;
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::process::{Command};

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
        
        let mut new_file_path = String::from(file_path);
        if let Some(last_dot_index) = new_file_path.rfind('.') {
            new_file_path.truncate(last_dot_index);
        }
        new_file_path.push_str(".sfsc");
        manager::write(&new_file_path, data, password)?;
    } else {
        let data = manager::read(file_path, password)?;
        
        let temp_dir = env::temp_dir();
        let mut temp_file_path = temp_dir.clone();
        temp_file_path.push(file_path);
        let mut temp_file = File::create(&temp_file_path)?;
        temp_file.write_all(&data)?;
        let mut notepad_process = Command::new("notepad.exe")
            .arg(&temp_file_path)
            .spawn()?;

        match notepad_process.wait() {
            Ok(_) => {
                fs::remove_file(&temp_file_path)?;
            }
            Err(e) => {
                eprintln!("Failed to wait for Notepad process: {}", e);
            }
        }
    }

    Ok(())
}
