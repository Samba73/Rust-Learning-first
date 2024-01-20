pub fn iterator() {
    let v = vec![11,21,31,41,51];
    // for item in v {
    //     println!("The item in vector v is {}", item);
    // }
    let mut v_iter: std::slice::Iter<'_, i32> = v.iter();
    println!("The iterator is {:#?}", v_iter);
    // println!("Total count is {}",v_iter.count());
    // println!("First item is {:#?}",v_iter.last());
    println!("The next item is {:#?}",v_iter.next());
    println!("The next item is {:#?}",v_iter.next());
    println!("The next item is {:#?}",v_iter.next());
    println!("The next item is {:#?}",v_iter.next());
    println!("The next item is {:#?}",v_iter.next());
    println!("The next item is {:#?}",v_iter.next());
}