use std::mem;

pub fn run(){

    let numbers: [i32;5] = [1,2,3,4,5];
    println!("the numbers are {:?}",numbers);
    println!("the first number is {}",numbers[0]);
    println!("the length of the array is {}", numbers.len());
    println!("the array occupies bytes : {}", mem::size_of_val(&numbers));

    //slicing the array
    let slice: &[i32] = &numbers[1..4];
    println!("the sliced array is {:?}",slice);

}