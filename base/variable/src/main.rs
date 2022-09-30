use std::collections::HashMap;

fn main() {
    test_cn_string();

    let mut s = String::from("hello world");
    let world = &s[6..s.len()];
    println!("{}", world); // correct
    s.clear();

    let s1 = String::from("hello");
    let s2 = String::from(",");
    let s3 = String::from("world");
    let s3 = format!("{}{} {}", s1, s2, s3);
    println!("{}", s3);

    let space = 123;
    println!("{}", &space);

    let mut aaa = String::from("aaa");
    let size = modify_str_size(&mut aaa);
    println!("{}, {}", &size, aaa);

    non_pointer_str(aaa);
    // 这里会报错，因为前面 aaa 传进去的非指针，而是整个 move in ，aaa 已经 invalid
    // println!("{}", aaa);

    let v: u16 = test_fn(666);
    println!("{}", &v);
    println!("{}, {}", cond_return(99), cond_return(101));

    let (a, b) = test_loop(2, 8);
    println!("{}, {}", a, b);

    test_arr();
    mut_str();

    let mut ssss = no_danle();
    ssss.push('1');
    println!("{}", ssss);

    let s = String::from("hello world");
    let index = first_word_index(&s);
    println!("{}", index);
    let first_word = &s[0..index];
    println!("{}", first_word);

    test_vec();

    println!("{}", test_num(1));
    println!("{}", test_str("1"));

    test_hashmap();
}

fn test_vec() {
    let mut v: Vec<u32> = Vec::new();
    v.push(1);
    v.push(2);
    println!("{:?}", v);

    let mut v2 = vec![1, 2, 3];
    v2.push(4);
    v2.push(5);
    v2[0] += 1;
    println!("{:?}", v2);
    for i in &mut v2 {
        *i += 1;
    }
    println!("{:?}", v2);

    #[derive(Debug)]
    enum MyEnum {
        A(String),
        B(u32),
        C(f32),
    }
    let v3 = vec![
        MyEnum::A(String::from("123")),
        MyEnum::B(1),
        MyEnum::C(2.0),
    ];
    println!("{:?}", v3);

    let v4 = vec![1, 2, 3];
    println!("{}", &v4[0]);
    match v4.get(0) {
        Some(n) => {
            println!("{}", n);
        }
        None => ()
    }
}

fn test_cn_string() {
    let s = String::from("吖猩测试");

    let s2 = &s[0..3];
    println!("{}, {}", s2, s2.len());

    for (i, c) in s.chars().enumerate() {
        if i == 0 {
            println!("{}", c);            
        } else {
            break;
        }
    }
}

fn first_word_index(w: &String) -> usize {
    let bytes = w.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    w.len()
}

fn no_danle() -> String {
    // s 在 scope 里定义，不能返回指针
    // 否则 scope 结束，s 的内存会释放，指针成了空指针
    let s = String::from("123");
    s
}

fn test_num(n: u32) -> u32 {
	match n {
    	1 | 2 => 1, // 1 或者 2 都返回 1
        _ => 0, // 兜底返回 0
    }
}

fn test_str(n: &str) -> u32 {
	match n {
    	"1" => 1, // 1
        _ => 0, // 兜底返回 0
    }
}

fn test_fn(v: u16) -> u16 {
    let a: u16 = {
        let b = 223;
        // 不能用 ; ，不然算成 statement，就不会赋值给 a
        b + v
    };

    return a;
}

fn non_pointer_str(bbb: String) {
    println!("{}", bbb);
}

fn modify_str_size(bbb: &mut String) -> usize {
    bbb.push_str("123123");

    // 不带 ; 也算 return
    bbb.len()
}

fn cond_return(b: u8) -> bool {
    return if b > 100 { true } else { false }
}

fn test_loop(mut a: u8, mut b: u8) -> (u8, u8) {
    // 类似于 while true 了
    loop {
        if a >= 10 {
            break;
        }

        a += 1;
    }

    while b < 10 {
        b += 1;
    }

    return (a, b);
}

fn test_arr() {
    let mut arr = [ 1, 2, 3, 4 ];
    arr.reverse();

    for el in arr.iter() {
        println!("el curr is {}", el);
    }

    for el2 in (1..5).rev() {
        println!("el2 curr is {}", el2);
    }

    let mut iter = arr.chunks(3);
    let v = iter.next().unwrap();
    println!("{}", v[0]);
}

fn mut_str() {
    // 如果 string 是可变的，那么就得用 String ，会塞进 heap 中
    let mut text = String::from("hello");
    text.push('1'); // 单引号代表 char
    text.push_str("222222"); // str

    // text 赋值给 text2 后会被标记为 invalid，不能再用 text
    // 保证在释放内存的时候，触发两次 drop
    let text2 = text;
    println!("{}", text2);

    // clone 才允许
    let text3 = text2.clone();
    println!("{}, {}", text2, text3);

    // literals 是不可变的
    // let text2 = "asdasd";
}

fn test_hashmap() {
    println!("\n\n===test_hashmap===");

    let mut h = HashMap::new();
    let key = String::from("h");
    h.insert(String::from("h"), &key);
    println!("{:?}", h);

    let k = vec![String::from("h"), String::from("b")];
    let v = vec![1, 2];
    let mut h2: HashMap<_, _> = k.into_iter().zip(v.into_iter()).collect();
    println!("{:?}", h2);

    let r = h2.get(&String::from("h"));
    match r {
        Some(n) => {
            println!("{}", n);
        },
        _ => (),
    }

    for (key, value) in &h2 {
        println!("{}: {}", key, value);
    }

    let r2 = h2.entry(String::from("h")).or_insert(0);
    *r2 += 1;
    println!("{}", r2);
}