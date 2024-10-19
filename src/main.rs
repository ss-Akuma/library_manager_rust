use std::io::{self, Write};
use std::collections::HashMap;
fn signUp(){
    println!("you've entered signup")
}


fn logIn(){
    println!("you've entered login")

}
    

fn main() {
    let mut user_info: HashMap<String, String> = HashMap::new();
    println!("### Welcome ###");
    println!("please choose your operation (login/signup/exit): ");
    
    loop{
        let mut operation = String::new();

        io::stdin().read_line(&mut operation).expect("error");

        io::stdout().flush().unwrap();

        let opr = operation.trim().to_lowercase();

        match opr.as_str() {
            "login" => {
                println!("please enter your username: ");
                let mut user_name = String::new();
                io::stdin().read_line(&mut user_name).expect("error");
                io::stdout().flush().unwrap();
                for (key,value) in user_info {
                    if user_name.as_str() == value.as_str(){
                        println!("2");
                        break;
                    }else{
                        println!("username not found");
                    }
                } 
                logIn();
                break;
            }
            "signup" => {
                signUp();
                break;
            }
            "exit" => {
                println!("Exiting...");
                std::process::exit(0);
            }
           _ => println!("invalid operation!")

        }
        
    }    
}
