pub fn run(){
    //primitive string
    let hello = "hello";

    //mutable string from string module, can be made mutable if mut is specified next to let
    let mut name = String::from("name");

    println!("{} {}! and length is : {}",hello,name,name.len());

    //push char
    name.push('_');
    //push string
    name.push_str("venky");
    //capacity in bytes
    println!("{}, len : {}, capacity: {}",name,name.len(),name.capacity());

    //string. ctrl+space for all kinds of functions of the string
    println!("replaced word : {}",name.replace("ven","ben"));

    //looping
    for word in name.split('_'){
        println!("{}",word);
    }

    //string with capacity
    let mut str = String::with_capacity(10);

    str.push('a');
    str.push('b');
    //use for testing if something equals something
    assert_eq!(str,"ab");
}