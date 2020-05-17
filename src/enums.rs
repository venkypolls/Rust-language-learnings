enum Movement{
    Left,
    Right,
    Up,
    Down
}

fn actions(m:Movement){
    match m {
        Movement::Up => println!("moving up"),
        Movement::Down => println!("moving down"),
        Movement::Left => println!("moving left"),
        Movement::Right => println!("moving right")

    }
}
pub fn run(){

    let avatar1 = Movement::Up;
    let avatar2 = Movement::Down;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Left;

    actions(avatar1);
    actions(avatar2);

}