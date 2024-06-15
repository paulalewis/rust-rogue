use std::io::{Read, Error};

/// Reads and returns a character, checking for gross input errors
/// readchar()
pub fn read_character() -> Result<char, Error> {
    let mut std_in = std::io::stdin();
    let mut buf = ['\n' as u8; 1];
    while buf[0] == '\n' as u8 || buf[0] == '\r' as u8 {
        std_in.read_exact(&mut buf)?;
    }
    return Ok(buf[0] as char);
}

/// Wait for the user to type a character
/// wait_for()
pub fn wait_for_character(character: char) -> Result<(), Error> {
    while read_character()? != character {}
    Ok(())
}

/// Wait for the user to type a character
/// wait_for()
pub fn wait_for_any_character() -> Result<(), Error> {
    read_character()?;
    Ok(())
}
