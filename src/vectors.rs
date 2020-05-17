use std::mem;

pub fn run(){

    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    numbers.push(6);
    numbers.push(7);
    numbers.pop();
    println!("the numbers are {:?}",numbers);
    println!("the first number is {}",numbers[0]);
    println!("the length of the vector is {}", numbers.len());
    println!("the vector occupies bytes : {}", mem::size_of_val(&numbers));

    //slicing the array
    let slice: &[i32] = &numbers[1..4];
    println!("the sliced array is {:?}",slice);

    for x in numbers.iter_mut(){
        *x *= 3;
    }
    println!("numbers - {:?}",numbers);

}