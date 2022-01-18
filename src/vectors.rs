//Vectors are resizable arrays

pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4]; // use the vec to define
    //same modules as arrays

    println!("{:?}", numbers);

    //add to a vector
    numbers.push(5);
    println!("{:?}", numbers);
    numbers.pop();
    println!("{:?}", numbers);

    //LOOPING
    for x in numbers.iter() {
        println!("Number: {}", x+1);
    }

}