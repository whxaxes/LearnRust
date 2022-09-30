use std::io::{ErrorKind, Write};
use std::{fs::remove_file, fs::File, io::Read};

fn main() {
    let file = "src/text2.txt";
    let mut f = File::open(&file).unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            println!("ignore not foud file");
            let mut nf = File::create(&file).unwrap();
            nf.write_all(b"hello world").unwrap();

            File::open(&file).unwrap()
        } else {
            panic!("other error");
        }
    });
    let mut str = String::new();
    let r = f.read_to_string(&mut str).expect("read file filed");
    println!("content is {}, size is {}", str, r);

    remove_file(&file).unwrap();

    let r2 = test_fn_once(|str| {
        let u: u32 = str.parse().expect("failed to parse to number");
        u
    });
    println!("{}", r2);
}


fn test_fn_once<F: Fn(String) -> T, T> (f: F) -> T{
    f(String::from("123"))
}