fn main() {
    let a = Box::new(2);
    let b = *a + 1;
    println!("{:?}", b);


    let mut arr = [0; 1000];
    let arr1 = arr;
    arr[0] = 1;

    println!("{:?}, {:?}", arr.len(), arr[0]);
    println!("{:?}, {:?}", arr1.len(), arr1[0]);

    let mut arr2 = Box::new([0; 1000]);
    (*arr2)[0] = 1;

    let mut arr3 = arr2;
    (*arr3)[1] = 1;

    println!("{:?}, {:?}", arr3[0], arr3[1]);
}
