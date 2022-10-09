// https://leetcode.cn/problems/longest-substring-without-repeating-characters/
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max = 0;
    let mut start = 0;
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        let mut j = start;
        while j < i {
            let cc = bytes[j];
            if b == cc {
                let nmax = i - start;
                if max < nmax {
                    max = nmax;
                }
                start = j + 1
            }
            j += 1;
        }
    }

    let nmax = bytes.len() - start;
    if max < nmax {
        max = nmax;
    }

    max as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(length_of_longest_substring(String::from("aab")), 2);
        assert_eq!(length_of_longest_substring(String::from(" ")), 1);
        assert_eq!(length_of_longest_substring(String::from("")), 0);
        assert_eq!(length_of_longest_substring(String::from("au")), 2);
        
        assert_eq!(length_of_longest_substring(String::from("abcabcbb")), 3);
        assert_eq!(length_of_longest_substring(String::from("bbbbb")), 1);
        assert_eq!(length_of_longest_substring(String::from("pwwkew")), 3);
    }
}
