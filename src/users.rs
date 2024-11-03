mod id_generator;
use std::collections::HashMap;
mod role;
use role::Roles;

pub struct User{
    username: String,
    password: String,
    id: String,
    role: Roles

}
impl User{
    fn new(username: String,password: String) -> Self{
        Self{
            username: username,
            password: password,
            id:id_generator::generate_custom_id(20),
            role:Roles::Guest
        }
    }
    pub fn get_username(&self) -> &str{
        &self.username
    }
    pub fn get_password(&self) -> &str{
        &self.password
    }
    pub fn set_username(&mut self,new_username: String){
        self.username = new_username
    }
    pub fn set_password(&mut self,new_password: String){
        self.password = new_password
    }
    //pub fn set_role(){}
    //pub fn get_role(){}
    pub fn signup(username: String,password: String,user_db: &mut HashMap<String,User>) -> bool{
        if user_db.contains_key(&username){
            return false;
        }
        let user = User::new(username, password);
        user_db.insert(user.get_username().to_string(), user);
        true
    }
    pub fn login(username: String,password: String,user_db: &HashMap<String,User>) -> bool{
        if let Some(user) = user_db.get(&username){
            return user.get_password() == password;
        }
        false
    }
    
}