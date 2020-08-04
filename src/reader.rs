use std::io;
use std::io::Read;
use std::string;

pub fn read_var_int<R: Read>(reader: &mut R) -> io::Result<i32> {
    const MORE_FLAG: u8 = 0b10000000;
    let mut int = 0i32;
    let mut num_read = 0;

    loop {
        let mut buf = [0];
        reader.read_exact(&mut buf)?;
        let byte = buf[0];

        int += ((byte & !MORE_FLAG) as i32) << (7 * num_read);

        num_read += 1;
        if num_read > 5 {
            return Err(io::Error::new(io::ErrorKind::Other, "VarInt is too big"));
        }

        if byte & MORE_FLAG == 0 {
            break;
        }
    }

    Ok(int)
}

// TODO: range check
pub fn read_string<R: Read>(
    reader: &mut R,
) -> io::Result<Result<String, string::FromUtf8Error>> {
    let len = read_var_int(reader)? as usize;
    let mut buf = vec![0; len];
    reader.read_exact(&mut buf)?;
    Ok(String::from_utf8(buf))
}

pub fn read_unsigned_short<R: Read>(reader: &mut R) -> io::Result<u16> {
    let mut buf = [0; 2];
    reader.read_exact(&mut buf)?;
    Ok(((buf[0] as u16) << 8) + buf[1] as u16)
}
