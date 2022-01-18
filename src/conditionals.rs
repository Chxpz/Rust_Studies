pub fn run(){
    let age: u8 = 32;
    let check: bool = true;

    if age >= 21 && check {
        println!("Ok");
    }else if age <21 && check {
        println!("Failed")
    }else{
        println!("Might could fail. Considering check is true?")
    }

    //or double pipe||
    //age >= 21 && check || <something>

    //shorthand no ternary operator. Just this
    let is_of_age = if age >=21 {true} else {false};
    println!("is of age: {}", is_of_age);

}