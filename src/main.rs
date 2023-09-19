mod modes;
use modes::{get_args,read,write,append,search,output};
use std::env;


fn main() { 
    let args: Vec<String> = env::args().collect();
    
    if args.len()==1{
        get_args().expect("Failed to get args.");
    }else if args.len() < 4 {
        eprintln!("Usage: cargo run -[r/w/a/s/o] text.txt Mypassword <Extra args>");
        return ();
    }

    let mode = &args[1];
    match mode.as_str() {
        "-r" => {
            read(args).expect("Failed to read.")
        }
        "-w" => {
            write(args).expect("Failed to write.")
        }
        "-a" => {
            append(args).expect("Failed to append.")
        }
        "-s" => {
            search(args).expect("Failed to search.")
        }
        "-o" => {
            output(args).expect("Failed to create output.")
        }
        _ => {
            eprintln!("Invalid mode: {}", mode)
        }
    }
}
