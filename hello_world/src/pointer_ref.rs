pub fn run() {
    // primitive arrays
    let arr1 = [1,2,3];
    let arr2 = arr1;
    println!("Arr1 and 2: {:?}", (arr1, arr2));

    // non primitives
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;
    println!("vec1 and 2: {:?}", (&vec1, vec2));
}