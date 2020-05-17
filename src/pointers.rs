pub fn run(){
    //primitve
    let arr1 = [1,2,3];
    let arr2 = arr1;
    println!("both the arrs {:?}",(arr1,arr2));

    //non primitive
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;
    println!("both thr vecs {:?}",(&vec1,vec2));
}