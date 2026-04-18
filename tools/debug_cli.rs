use std::io::{self, Write};

pub fn start_debug_shell() {
    println!("IPv8 Debug CLI started. Type 'exit' to quit.");

    loop {
        print!("ipv8-debug> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let cmd = input.trim();

        match cmd {
            "exit" => break,
            "help" => println!("Commands: help, exit, status"),
            "status" => println!("IPv8 system running (simulated)"),
            _ => println!("Unknown command: {}", cmd),
        }
    }
}
