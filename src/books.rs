mod id_generator;
pub struct Book{
    name:String,
    year:u32,
    author:String,
    id:String
}

impl Book {
    pub fn new(name:String,year:u32,author:String) -> Self{
        Self{
            name:name,
            year:year,
            author:author,
            id:id_generator::generate_custom_id(10)
        }
    }
    pub fn get_name(&self) -> &str{
        &self.name
    }
    pub fn get_year(&self) -> &u32{
        &self.year                                                                                    
    }
    pub fn get_author(&self) -> &str{
        &self.author    
    }
    pub fn get_id(&self) -> &str{
        &self.id    
    }
    pub fn set_name(&mut self,new_name:String){
        self.name = new_name;
    }
    pub fn set_year(&mut self,new_year:u32){
        self.year = new_year;
    }
    pub fn set_author(&mut self,new_author:String){
        self.author = new_author;
    }
    // pub fn set_id(&self){

    // }    


}