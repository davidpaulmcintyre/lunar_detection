use std::env;  
use sysinfo::{ProcessExt, System, SystemExt}; 


fn main() {
    println!("Program starting");

    is_lunar_running();
}

fn has_python_env_var() -> bool {
    let python_env_var = "PYGAME_HIDE_SUPPORT_PROMPT";
    let is_present = match env::var(python_env_var) {
        Ok(_) => true,
        Err(_) => false 
    };
    is_present
} 

fn is_lunar_running() { 
    if has_python_env_var() { 
        let mut is_detected = false;
        let  sys = System::new_all(); 
        for (_pid, process) in sys.processes() {
            let _process_name = process.name();
            if _process_name == "python3" {
                // lunst.py script is designed so it has to be run from same dir
                let cmd = process.cmd(); 
                for token in cmd.iter(){ 
                    if token == "lunar.py" {
                        let path = process.cwd().display().to_string(); 
                        let mut directories: Vec<&str> = path.split('/').collect();
                        let dir = directories.pop().unwrap(); 
                        // println!("dir {}", dir);
                        if dir.to_lowercase() == "lunar" {
                            println!("Klling the process"); 
                            // kill the process running lunar.py
                            process.kill();
                            is_detected = true;

                        }
                    }

                } 
            }
        }
        if is_detected {
            println!("Script detected"); 
        } else {
            println!("Script not detected"); 
        }
    }
} 