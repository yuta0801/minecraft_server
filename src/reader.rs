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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_var_int_success() {
        fn test_read_var_int(bytes: &[u8], expected: i32) {
            let mut reader: &[u8] = bytes;
            assert_eq!(read_var_int(&mut reader).unwrap(), expected);
        }

        test_read_var_int(&[0x00], 0);
        test_read_var_int(&[0x01], 1);
        test_read_var_int(&[0x02], 2);
        test_read_var_int(&[0x7f], 127);
        test_read_var_int(&[0x80, 0x01], 128);
        test_read_var_int(&[0xff, 0x01], 255);

        // not suppoeted
        // test_read_var_int(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x07], 2147483647);
        // test_read_var_int(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x0f], -1);
        // test_read_var_int(&[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x08], -2147483648);
    }

    #[test]
    fn read_string_success() {
        fn test_read_string(bytes: &[u8], expected: String) {
            let mut reader: &[u8] = bytes;
            assert_eq!(read_string(&mut reader).unwrap().unwrap(), expected);
        }

        test_read_string(&[4, 116, 101, 115, 116], "test".to_string())
    }

    #[test]
    fn read_unsigned_short_success() {
        fn test_read_unsigned_short(bytes: &[u8], expected: u16) {
            let mut reader: &[u8] = bytes;
            assert_eq!(read_unsigned_short(&mut reader).unwrap(), expected);
        }

        test_read_unsigned_short(&[0u8, 0u8], 0);
        test_read_unsigned_short(&[0u8, 0x1u8], 1);
        test_read_unsigned_short(&[0u8, 0xffu8], 255);
        test_read_unsigned_short(&[0xffu8, 0xffu8], 65535);
    }
}
