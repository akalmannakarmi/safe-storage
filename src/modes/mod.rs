mod manager;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self,Write};
use std::process::{Command};
use std::env;

pub fn get_args() -> Result<Vec<String>, Box<dyn Error>> {
    let mut args: Vec<String> = Vec::new();

    let get_input = |prompt: &str| -> String {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().to_string()
    };

    args.push(get_input("Enter Mode(-[r/w/a/s/o]):"));
    args.push(get_input("Enter File Name(text.txt):"));
    args.push(get_input("Enter File Password:"));

    println!("Enter Extra args (enter an empty line to finish):");
    while let Some(line) = Some(get_input("")) {
        if line.as_str().is_empty() {
            break;
        }
        args.push(line);
    }

    Ok(args)
}


pub fn read(args:Vec<String>) -> Result<(), Box<dyn Error>>{
    let file_path = args[2].as_str();
    let password = args[3].as_str();

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
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to wait for Notepad process: {}", e);
            Err(Box::new(e))
        }
    }
}

pub fn write(args:Vec<String>) -> Result<(), Box<dyn Error>>{
    let file_path = args[2].as_str();
    let password = args[3].as_str();

    let data = fs::read(file_path)?;
    
    let mut new_file_path = String::from(file_path);
    if let Some(last_dot_index) = new_file_path.rfind('.') {
        new_file_path.truncate(last_dot_index);
    }
    new_file_path.push_str(".sfsc");
    Ok(manager::write(&new_file_path, data, password)?)
}

pub fn append(args:Vec<String>) -> Result<(), Box<dyn Error>>{
    let file_path = args[2].as_str();
    let password = args[3].as_str();

    let mut data:Vec<u8> = manager::read(file_path, password)?;
    data.push(b'\n');

    if args.len()>4{
        let add_data = fs::read(args[4].as_str())?;
        data.extend(add_data);
    }else{
        println!("Data to add (type 'exit()' to finish):");
        let mut input = String::new();
        loop {
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            if input.trim() == "exit()" {
                break;
            }
            data.extend(input.as_bytes());
        }
    }
    manager::write(&file_path, data, password)?;
    Ok(())
}

pub fn search(args:Vec<String>) -> Result<(), Box<dyn Error>>{
    let file_path = args[2].as_str();
    let password = args[3].as_str();

    
    let mut input = String::new();
    let search_data: &str = if args.len() > 4 {
        args[4].as_str()
    } else {
        println!("Data to Search:");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim()
    };

    let data: Vec<u8> = manager::read(file_path, password)?;
    let mut found_data: Vec<u8> = Vec::new();

    let data_string = String::from_utf8_lossy(&data);

    for line in data_string.lines() {
        if line.contains(search_data) {
            found_data.extend(line.as_bytes());
            found_data.push(b'\n');
        }
    }

    let temp_dir = env::temp_dir();
    let mut temp_file_path = temp_dir.clone();
    temp_file_path.push(file_path);
    let mut temp_file = File::create(&temp_file_path)?;
    temp_file.write_all(&found_data)?;
    let mut notepad_process = Command::new("notepad.exe")
        .arg(&temp_file_path)
        .spawn()?;

    match notepad_process.wait() {
        Ok(_) => {
            fs::remove_file(&temp_file_path)?;
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to wait for Notepad process: {}", e);
            Err(Box::new(e))
        }
    }
}

pub fn output(args:Vec<String>) -> Result<(), Box<dyn Error>>{
    let file_path = args[2].as_str();
    let password = args[3].as_str();

    let mut input = String::new();
    let output_path: &str = if args.len() > 4 {
        args[4].as_str()
    } else {
        println!("Output file:");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim()
    };

    let data = manager::read(file_path, password)?;
    fs::write(output_path,data)?;
    Ok(())
}