use std::fs::File;
use std::io::{self, BufReader, BufWriter, ErrorKind, Read, Write};
use std::io::{Read, Write};
use std::path::Path;

fn copy_file<P: AsRef<Path>>(source_file: P, target_file: P) -> io::Result<()> {
    let input_file = File::open(source_file)?;
    let mut reader = BufReader::new(input_file);

    let output_file = File::create(target_file)?;
    let mut writer = BufWriter::new(output_file);
    copy(&mut reader, &mut writer)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let input_file = "D:/rust_homework/src/Day05/copy-file/src/test01.pdf";
    let optput_file = "D:/rust_homework/src/Day05/copy-file/src/test02.pdf";
    copy_file(input_file, optput_file)?;
    Ok(())
}

fn copy<R: Read, W: Write>(reader: &mut R, writer: &mut W) -> io::Result<u64> {
    const BUFFER_SIZE: usize = 1024 * 8;
    let mut buffer = [0u8; BUFFER_SIZE];
    let mut write_len = 0u64;
    loop {
        let read_len = match reader.read(&mut buffer) {
            Ok(0) => return Ok(write_len),
            Ok(len) => len,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        writer.write_all(&buffer[..read_len])?;
        write_len += read_len as u64;
    }
}