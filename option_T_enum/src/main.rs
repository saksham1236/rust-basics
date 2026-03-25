fn main() {
    let count = [5, 4, 3, 2, 1];
    let number = count.get(5);
    // let number = number.unwrap_or(&0) + 1;//Unwrap or needs a reference as the same data type
    println!("number is {:?}", number);//Returns value none

    let number = match number {
        Some(number) => number + 1,
        None => 0
    };
    println!("number is {:?}", number);//Returns value none
}
