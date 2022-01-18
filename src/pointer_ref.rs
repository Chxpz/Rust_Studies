//reference pointers - point to a resource in memory

pub fn run(){
    //primative array - or any primitive
    let arr1 = [1,2,3];
    let arr2 = arr1;

    //With non-primitives, if you assign another variable to a piece of data, the first
    // variable will no longer hold that value. You will need to use a reference (&) to point to the
    // resource

    //vector is not a primitive
    let vec1 = vec![1,2,3];
    //let vec2 = vec1; wrong
    let vec2 = &vec1;

    println!("Values of Vectors: {:?}", (&vec1, vec2));



}