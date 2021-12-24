use std::fs::File;
use std::io::ErrorKind;

pub fn open(filename: &str, init_callback: &dyn Fn(File) -> File) -> File {
  let file = File::open(filename);
  return match file {
    Ok(f) => f,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create(filename) {
        Ok(fc) => init_callback(fc),
        Err(e) => panic!("Problem creating the file: {:?}", e),
      },
      other_error => {
        panic!("Problem opening the file: {:?}", other_error)
      }
    },
  };
}
