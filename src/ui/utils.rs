/// For printfs: if string starts with a vowel, return "n" for an "an".
/// vowelstr
pub fn vowelstr(str: &str) -> &str {
	match str.chars().next() {
		Some('a') | Some('A') | Some('e') | Some('E') | Some('i') | Some('I') | Some('o') | Some('O') | Some('u') | Some('U') => "n",
		_ => "",
	}
}
