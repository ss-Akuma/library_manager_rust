    use std::io::{self, Write};
    use std::collections::HashMap;
    fn sign_up(){
        println!("you've entered signup")
    }


    fn log_in(user_info: &HashMap<String,String>){
        println!("you've entered login");
        println!("please enter your username: ");
        let mut user_name = String::new();
        io::stdin().read_line(&mut user_name).expect("error");
        let user_name = user_name.trim();
        
        match user_info.get(user_name){
            Some(password) => {
                println!("please enter your password: ");
                let mut input_password = String::new(); 
                io::stdin().read_line(&mut input_password).expect("error");
                let input_password = input_password.trim();
                if password == input_password{
                    println!("you've logged in")
                }else{
                    println!("password incorrect!")
                }
                
            },
            None => println!("username not found!") 
        }
            

    }
        

    fn main() {
        let mut  user_info: HashMap<String, String> = HashMap::new();
        user_info.insert("ali".to_string(), "password123".to_string());
        println!("### Welcome ###");
        println!("please choose your operation (login/signup/exit): ");
        
        loop{
            let mut operation = String::new();

            io::stdin().read_line(&mut operation).expect("error");

            io::stdout().flush().unwrap();

            let opr = operation.trim().to_lowercase();

            match opr.as_str() {
                "login" => {
                    
                    log_in(&user_info);
                    break;
                }
                "signup" => {
                    sign_up();
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
