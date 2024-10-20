use std::io::{self, Write};
use std::collections::HashMap;

fn sign_up(user_info: &mut HashMap<String,String>){
    println!("you've entered signup");

    loop{
        println!("please enter your username: ");

        let mut user_name = String::new();
        io::stdin().read_line(&mut user_name).expect("error");
        let user_name = user_name.trim().to_string();

        if user_info.contains_key(&user_name) {
            println!("username exists!");
        }else {
            println!("please enter your password: ");

            let mut input_password = String::new(); 
            io::stdin().read_line(&mut input_password).expect("error");
            let input_password = input_password.trim().to_string();

            user_info.insert(user_name, input_password);
            println!("you've signed up succesfully!");
            return;
        }
    }
}

fn log_in(user_info: &HashMap<String,String>) -> bool{
    println!("you've entered login");
    println!("please enter your username: ");

    let mut user_name = String::new();
    io::stdin().read_line(&mut user_name).expect("error");
    let user_name = user_name.trim();
    
    match user_info.get(user_name){
        Some(password) => {
            loop {
                println!("please enter your password: ");
                let mut input_password = String::new(); 
                io::stdin().read_line(&mut input_password).expect("error");
                let input_password = input_password.trim();

                if password == input_password{
                    println!("you've logged in");
                    return true
                    
                }else{
                    println!("password incorrect!")
                }
            }
        },
        None => {
            println!("username not found!");
            false
        } 
    }
}
    

fn main() {
    let mut  user_info: HashMap<String, String> = HashMap::new();
    let mut _is_logged_in = false;
    user_info.insert("ali".to_string(), "password123".to_string());

    while !_is_logged_in{
        println!("please choose your operation (login/signup/exit): ");
        let mut operation = String::new();
        io::stdin().read_line(&mut operation).expect("error");
        io::stdout().flush().unwrap();
        let opr = operation.trim().to_lowercase();

        match opr.as_str() {
            "login" => {
                _is_logged_in = log_in(&user_info);
            }
            "signup" => {
                sign_up(&mut user_info);
                _is_logged_in = log_in(&user_info);
            }
            "exit" => {
                println!("Exiting...");
                std::process::exit(0);
            }
            _ => println!("invalid operation!")

        }
            
         
        
    }
    println!("Welcome to the system!");
       
}
