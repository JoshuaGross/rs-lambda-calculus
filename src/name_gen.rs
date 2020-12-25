pub struct NameGen {
    pub curr_ident: String
}

fn shift(input: &str) -> Result<String, String> {
    let mut carry: i16 = 1;
    let mut buf = Vec::with_capacity(input.len());
    for byte in input.bytes().rev() {
        match byte {
            b'a'..=b'z' => {
                let mut val = byte as i16 + carry;
                if val > b'z' as i16 {
                    carry = (val as i16) - (b'z' as i16);
                    val = b'a' as i16
                } else {
                    carry = 0
                }
                buf.push(val as u8);
            }
            _ => Err("Error")?,
        }
    }
    if carry != 0 {
        buf.push(b'a' as u8);
    }
    buf.reverse();
    let out = String::from_utf8(buf)
        .expect("we are sure that all bytes fall into ASCII range");
    Ok(out)
}

impl NameGen {
    pub fn next(&mut self) -> Result<&str, String> {
        self.curr_ident = shift(&self.curr_ident)?;
        Ok(&self.curr_ident)
    }
}
