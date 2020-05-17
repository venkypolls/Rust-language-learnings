
pub fn run(){
    greetings("hello","venky");
    println!("{} + {} = {}",2,5,add(2,5));
    let six = 6;
    //closure functions, in which can use outside variables like six here
    let clo_sum = |n1:i32,n2:i32| n1+n2+six;
    println!("closure sum : {}", clo_sum(2,4));
}
fn greetings(str1:&str, str2:&str){
    println!("{} {} !",str1,str2);
}
fn add(x:i32,y:i32) -> i32{
    x+y
}