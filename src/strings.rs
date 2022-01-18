pub fn run(){
    let hello = String::from("Hello");
    println!("Lenght {}", hello.len());

    let mut _hello = String::from("Hello ");

    _hello.push('W'); //just for one char
    _hello.push_str("orld");// more than one
    println!("{}", _hello);

    //contains a substring
    println!("Contains World {}", _hello.contains("orld"));

    //Replace
    println!("Replace World {}", _hello.replace("World","There"));

    //loop through string by whitespace
    for x in _hello.split_whitespace(){
        println!("{}", x);
    }

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    assert_eq!(2, s.len());//just through a message when an error occurs

    println!("{}", s);

}