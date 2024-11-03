use std::io::{self, Write};
use std::collections::HashMap;
use library_manager_rust::users::User;

fn operations(){
    println!("### please choose your operation ###
    \nsearch_book\nedit_book\nborrow_book\nadd_book\ndelete_book\n
edit_user\nlogout\nexit :");
            let mut operation = String::new();
            io::stdin().read_line(&mut operation).expect("error");
            io::stdout().flush().unwrap();
            let opr = operation.trim().to_lowercase();
    
            match opr.as_str() {
                "search_book" => {
                    search_book()
                }
                "edit_book" => {
                    edit_book()
                }
                "borrow_book" => {
                    borrow_book()
                }
                "add_book" => {
                    add_book()
                }
                "delete_book" => {
                    delete_book()
                }
                "edit_user" => {
                    edit_user()
                }
                "logout" => {
                    logout()
                }
                "exit" => {
                    println!("Exiting...");
                    std::process::exit(0);
                }
                _ => println!("invalid operation!")
    
            }
}


//fn 
fn main() {
    let mut  user_db: HashMap<String, User> = HashMap::new();
    let mut _is_logged_in = false;

    while !_is_logged_in{
        let mut operation = String::new();

        println!("please choose your operation (login/signup/exit): ");
        io::stdin().read_line(&mut operation).expect("error");
        io::stdout().flush().unwrap();

        let opr = operation.trim().to_lowercase();

        match opr.as_str() {
            "login" => {
                let mut user_name = String::new();
                let mut input_password = String::new();

                println!("### login ###");

                println!("please enter your username: ");
                io::stdin().read_line(&mut user_name).expect("error");
                let user_name = user_name.trim().to_string();
        
                println!("please enter your password: ");
                io::stdin().read_line(&mut input_password).expect("error");
                let input_password = input_password.trim().to_string();

                let login_succes = User::login(user_name, input_password, &mut user_db);
                
                if login_succes{
                    _is_logged_in = true;
                }
                else {
                    println!("username or password not valid!");
                }
            }
            "signup" => {

                println!("### signup ###");

                loop{
                    let mut user_name: String = String::new();
                    let mut input_password: String = String::new(); 

                    println!("please enter your username: ");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut user_name).expect("error");
                    let user_name = user_name.trim().to_string();
            
                    println!("please enter your password: ");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut input_password).expect("error");
                    let input_password = input_password.trim().to_string();
            
                    let signup_succes = User::signup(user_name,input_password,&mut user_db);

                    if signup_succes {
                        println!("signup succesful!");
                        break;
                    }
                    else {
                        println!("Signup failed: User already exists.")
                    }
                }
                println!("### login ###");

                println!("please enter your username: ");
                let mut user_name = String::new();
                io::stdin().read_line(&mut user_name).expect("error");
                let user_name = user_name.trim().to_string();
        
                println!("please enter your password: ");
                let mut input_password = String::new(); 
                io::stdin().read_line(&mut input_password).expect("error");
                let input_password = input_password.trim().to_string();

                let login_succes = User::login(user_name, input_password, &mut user_db);
                
                if login_succes{
                    _is_logged_in = true;
                }
                else {
                    println!("username or password not valid!");
                }
            }
            "exit" => {
                println!("Exiting...");
                std::process::exit(0);
            }
            _ => println!("invalid operation!")

        }    
    }
    println!("### Welcome to the system! ###");
    operations();
    
}
