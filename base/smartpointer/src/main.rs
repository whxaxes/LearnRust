use std::rc::Rc;
use std::cell::{ Cell, RefCell };
use global_util::TestUtil;

fn main() {
    TestUtil::wrap_with_label("test_box", test_box);
    TestUtil::wrap_with_label("test_rc", test_rc);
    TestUtil::wrap_with_label("test_cell", test_cell);
}

fn test_box() {
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

fn test_rc() {
    let a = Rc::new(String::from("123123"));
    println!("a count {:?}", Rc::strong_count(&a));

    let b = a.clone();
    println!("a count {:?}", Rc::strong_count(&a));
    println!("b count {:?}", Rc::strong_count(&b));
    println!("{}, {}", a, b);

    let c = Rc::clone(&a);
    println!("c count {:?}", Rc::strong_count(&c));
}

fn test_cell() {
    let a = Cell::new(1);
    let b = a.get();
    a.set(2);
    println!("{:?}, {:?}", b, a.get());

    let d = Rc::new(RefCell::new(String::from("123123")));
    let e = d.clone();

    e.borrow_mut().push_str("6666");
    d.borrow_mut().push_str("string");
    d.borrow_mut().push_str("stringa");

    println!("{:?}", d);
}