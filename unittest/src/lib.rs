pub fn mylib() -> u32 {
	1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("it works");
        assert_eq!(mylib(), 1);
    }

    #[test]
    #[should_panic(expected = "should throw err")]
    fn it_should_panic() {
        panic!("should throw err");
    }

    #[test]
    fn test_panic() {
        if 2 + 2 != 4 {
            panic!("123")
        }
    }

    #[test]
    fn test_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("111"))
        }
    }

    #[test]
    #[ignore]
    fn it_should_panic_2() {
        panic!("should throw err");
    }
}
