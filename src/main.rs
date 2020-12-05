use std::fs;
use std::io::prelude::*;

fn main() {
  // let paths = .unwrap();
  let paths = fs::read_dir("./");
  let paths = match paths {
    Ok(n) => n,
    Err(e) => panic!("Error: {}", e),
  };

  for path in paths {
    let path = match path {
      Ok(n) => n.path().display().to_string(),
      Err(e) => panic!("Error: {}", e),
    };
    if path.contains(".mb"){
      println!("Name: {}", path);
      let mut file = match fs::File::open(path){
        Ok(n) => n,
        Err(e) => panic!("Error: {}", e),
      };
      let mut contents = String::new();
      
      match file.read_to_string(&mut contents) {
        Ok(n) => n,
        Err(e) => panic!("Error: {}", e),
      };

      print!("{}",contents);
    }
  }
}
