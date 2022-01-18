//Here structs works like classes
struct Person {
    first_name: String,
    last_name:String
}

//this is like methods in classes
impl Person{
    fn new(first:&str, last:&str) -> Person{
        Person{
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    //GET FULL NAME
    fn full_name(&self) -> String{
        format!("{} {}", self.first_name, self.last_name)
    }

    //set last name
    fn set_last_name(&mut self, last:&str) {
        self.last_name = last.to_string();
    }

    //Name tp tuple
    fn to_tuple(self) -> (String, String){
       (self.first_name, self.last_name) 
    }

}


pub fn run(){
    let mut p = Person::new("John", "Doe");
    println!("Person {} {}", p.first_name, p.last_name);
    println!("Person {} ", p.full_name());
    
    p.set_last_name("Willians");
    println!("Person {} ", p.full_name());
    println!("Person tuple {:?} ", p.to_tuple());

}