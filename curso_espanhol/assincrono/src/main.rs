fn main() {
  //IO Bound
  /* std::thread::spawn(|| {
    let _io_bound = read_file("foo.txt");
    println!("{_io_bound}");
  }); */
  //Não funciona por falta do async runtime
  || async {
    let _assync = read_file_assync("foo.txt").await;
    println!("{_assync}");
  };
}

//IO Bound
fn _read_file(filename: &str) -> String {
  println!("Reading file: {}", filename);
  std::thread::sleep(std::time::Duration::from_secs(2));
  "Hola".to_string()
}
async fn read_file_assync(filename: &str) -> String {
  println!("Reading file: {}", filename);
  std::thread::sleep(std::time::Duration::from_secs(2));
  "Hola".to_string()
}
