pub fn leftpad(pad: u64, ch: char, mut string: String) -> String {
    if pad != 0 {
        let char = String::from(ch);
        let mut temp = String::with_capacity(pad as usize + string.len());
        for _i in 0..pad {
            temp.push_str(char.as_str());
        }
        temp.push_str(string.as_str());
        temp
    } else {
        string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = leftpad(5, ' ', String::from("hello"));
        assert_eq!("     hello", result);
    }
    #[test]
    fn it_works2() {
        let result = leftpad(5, 'l', String::from("hello"));
        assert_eq!("lllllhello", result);
    }
}
