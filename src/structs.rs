
struct Person{
    first_name : String,
    last_name : String
}
impl Person{
    fn new_name(fname:&str,lname:&str) -> Person{
        Person{
            first_name:fname.to_string(),
            last_name:lname.to_string()
        }
    }
    fn get_full_name(&self) -> String{
        format!("{} {}",self.first_name,self.last_name)
    }
    fn set_last_name(&mut self, name:&str){
        self.last_name = name.to_string();
    }
    fn to_tuple(self) -> (String,String){
        (self.first_name,self.last_name)
    }
}
struct Names(i32,i32,i32);
struct Color{
    red :i32,
    blue:i32,
    green:i32
}

pub fn run(){
    //traditional struct
    let mut c = Color{
        red : 255,
        blue : 50,
        green : 0
    };
    c.green = 50;
    println!("color {} {} {}",c.red,c.blue,c.green);
    //touple struct
    let n = Names(100,35,0);
    println!("names {} {} {}", n.0,n.1,n.2);
    //access implementations
    let mut p = Person::new_name("venky","polls");
    println!("name : {} {}",p.first_name,p.last_name);
    // full name returned
    println!("full name : {}",p.get_full_name());
    //set last name
    p.set_last_name("polepally");
    println!("changed name : {}",p.get_full_name());
    //print tuple
    println!("tuple name : {:?}",p.to_tuple());

}