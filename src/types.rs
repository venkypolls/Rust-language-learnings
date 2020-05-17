pub fn run(){

    //default integer val
    let x = 1;
    //default float val
    let f = 2.5;
    //typed integer 64 bit
    let x1:i64 = 50505050;
    //max val
    println!("max val : {}",std::i32::MAX);
    println!("max val : {}",std::i64::MAX);
    // boolean
    let tr = true;
    //char
    let a = 'a';
    let emoji = '\u{1F600}';
    println!("{:?}",(x,f,x1,tr,a,emoji));


}