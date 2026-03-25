fn sum_boxes<T:std::ops::Add<Output = T> + Clone>(a:&Box<T>, b:&Box<T>) -> Box<T>{
    return Box::new(*(a.clone()) + *(b.clone()))
}

fn compare_boxes<T:std::cmp::PartialOrd + Clone>(a:&Box<T>, b:&Box<T>) -> Box<T> {
    if *a > *b {
        a.clone()
    } else {
        b.clone()
    }
}

fn main() {
    let one = Box::new(1);
    let two: Box<i32> = Box::new(2);
    assert_eq!(*sum_boxes(&one, &two), 3);
    assert_eq!(*compare_boxes(&one, &two), 2);
    let pi = Box::new(3.14159);
    let e = Box::new(2.71828);
    assert_eq!(*sum_boxes(&pi, &e), 5.85987);

    
    println!("Tests passed!");
}
