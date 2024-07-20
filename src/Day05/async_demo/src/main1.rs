use async_std::{io::{ReadExt, WriteExt}, net};
async fn cheap_request(host: &str, port:u16, path:&str) -> std::io::Result<String> {
  let mut socket = net::TcpStream::connect((host, port)).await?;

  let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
  socket.write_all(request.as_bytes()).await?;
  socket.shutdown(net::Shutdown::Write)?;

  let mut response = String::new();
  socket.read_to_string(&mut response).await?;
  Ok(response)
}

#[async_std::main]
async fn main() {
  let result = cheap_request("baidu.com", 80, "/").await.unwrap();
  println!("{}", result);
  // match result {
  //   Ok(content) => println!("response content = \n{}", content),
  //   Err(e) => println!("error occured: {:?}", e)
  // }
}