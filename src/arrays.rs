//Arrays - Fixed list where elements are the same data type

pub fn run(){
    let mut numbers: [i32;5] = [1,2,3,4,5];

    //Re-assing value
    numbers[2] = 20;
    println!("{:?}", numbers);
    
    //lenght
    println!("Length {}", numbers.len());

    println!("{:?}", numbers);
    //get single val
    println!("{}", numbers[0]);

    //get slice
    let mut slice: &[i32] = &numbers;
    println!("slice: {:?}", slice);

    slice = &numbers[1..3];
    println!("slice: {:?}", slice);


}