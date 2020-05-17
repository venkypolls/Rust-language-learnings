pub fn run(){
    //conditionals

    let x = 19;
    if x==19{
        println!("they're both equal");
    }
    //shorthand if
    let age =  if x>=18 { 18}else {0};
    println!("age is {}",age);


    //loops
    //infinite loop
    let mut count = 0;
    loop{
        count+=1;
        println!("the count is : {}",count);

        if count >= 20 {
            break;
        }

    }
    //while loop
    // while count < 100 {
    //     if count %15 == 0 {
    //         println!("fizzbuzz")
    //     }else if count%3 == 0 {
    //         println!("fizz");
    //     }else if count %5 == 0{
    //         println!("buzz");
    //     }else {
    //         println!("{}",count);
    //     }
    //     count+=1;
    // }

    for count in 0..100 {
        if count %15 == 0 {
            println!("fizzbuzz")
        }else if count%3 == 0 {
            println!("fizz");
        }else if count %5 == 0{
            println!("buzz");
        }else {
            println!("{}",count);
        }

    }



}