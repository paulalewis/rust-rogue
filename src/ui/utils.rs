/// For printfs: if string starts with a vowel, return "n" for an "an".
/// vowelstr
pub fn vowelstr(str: &str) -> &str {
	match str.chars().next() {
		Some('a') | Some('A') | Some('e') | Some('E') | Some('i') | Some('I') | Some('o') | Some('O') | Some('u') | Some('U') => "n",
		_ => "",
	}
}

pub fn center_text_index(str: &str) -> usize {
	28 - ((str.len() + 1) / 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn center_text_index_zero_len() {
        let len = center_text_index("");
		assert_eq!(len, 28);
    }
    
    #[test]
    fn center_text_index_one_len() {
        let len = center_text_index("1");
		assert_eq!(len, 27);
    }
    
    #[test]
    fn center_text_index_21_len() {
        let len = center_text_index("live long and prosper");
		assert_eq!(len, 17);
    }
    
    #[test]
    #[should_panic]
    fn center_text_index_60_len() {
        let len = center_text_index("||||||||| ||||||||| ||||||||| ||||||||| ||||||||| ||||||||| ");
		assert_eq!(len, 0);
    }
}
