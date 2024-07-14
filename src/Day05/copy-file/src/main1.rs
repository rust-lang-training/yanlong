

fn copy<R, W>(reader: R, writer: W) -> io::Result<u64>
  where R:Read, W:Write
{
  const BUF_SIZE: usize = 1024 * 64;
  let mut buf = [0u8, BUF_SIZE];
  let mut write_len = 0u64;

  loop {
    let len = match reader.read(&mut buf) {
      Ok(0) => return Ok(write_len),
      len => len,
      Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
      Err(e) => return Err(e)
    };
    writer.write_all(&buf[..len])?;
    writer_len += len as u64;
  }
}

fn copy_file<P: AsRef<Path>>(soure_file: P, target_file: P) -> io::Result<()> {
  let input_file = file_open(soure_file)?;
  let mut reader = BufReader::new(input_file);

  let output_file = File::create(target_file)?;
  let mut writer = BufWriter::new(output_file);

  copy(&mut reader, &mut writer)?;
  Ok(())
}

fn main() {
  let input_filename = "/D:/rust_homework/src/Day05/copy-file/src/test.pdf";
  let output_filename = "/D:/rust_homework/src/Day05/copy-file/src/test.pdf";
  copy_file(input_filename, output_filename)?;
  Ok(())
}


