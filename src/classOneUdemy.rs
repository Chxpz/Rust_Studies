#![allow(dead_code)]
#![allow(unused_variables)]
#[warn(non_snake_case)]

use std::mem; //used to deal with memory
//pub exports the function
pub fn classone() {
    let a:u8 = 123; //u = usigned, 8bits, 0 - 255 imutable
    let mut b:i8 = 0; //mut to make the variable mutable. i =signed. -128 -- 127
    println!("a value is {}", a);
    println!("b value before is {}", b);
    b = 42;
    println!("b value after is {}", b);
    let c = 123456789;
    println!("value of c is {} and c takes up {} bytes", c, mem::size_of_val(&c));
    //u8,u16,u32,u64,i8,i16....

    //usize, isize
    let z: isize =123;
    let size_of_z:usize = mem::size_of_val(&z);
    println!("z = {}, takes uo {}, {}-bits OS", z, size_of_z, size_of_z*8);



}