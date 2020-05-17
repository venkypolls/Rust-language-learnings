
pub fn run(){
    let name = "Brad";
    //how to make a variable mutable
    let mut age = 20;
    age += 1;
    println!("my name is {} and my age is {}", name,age);
    //using const
    const ID:i8 = 004;
    println!("my id is Id: {}",ID);
    //defining touple vaiables
    let (my_name,my_age) = ("venky", 53);
    println!("my name is {0}, my age is {1}",my_name,my_age);



}