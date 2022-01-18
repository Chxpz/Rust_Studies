pub fn run(){
    greeting("Hello","Tutu");
    //Bind functions values to variables
    let get_sum = add(5,5);
    println!("Sum: {}", get_sum);

    //Closure
    //can use variables outsie of the block
    let n3: i32 =10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C Sum: {}", add_nums(3,3));

}

fn greeting(greet: &str, name: &str){
    println!("{} {}", greet,name);
}

//the -> means return
//if a function return something do not use the ; at the end
fn add (n1: i32, n2: i32) -> i32 {
    n1 + n2
}