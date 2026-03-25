fn main() {
    let mut astronauts:Vec<String> = Vec::new();
    astronauts.push(String::from("Shepard"));//Same as arrays vectors can be pushed
    astronauts.push(String::from("Grissom"));
    astronauts.push(String::from("Glenn"));
    println!("astronauts is {:?}", astronauts);

    let last = astronauts.pop();//pops the last item
    println!("last is {:?}", last);

    // let third = &astronauts[2];//Need to reference items as they are stored on heap
    let third = astronauts.get(2);//get is safer option because it returns options enum
    println!("{:?}", third);

    let countdown = vec! [5, 4, 3, 2, 1];   
}
