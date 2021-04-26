

pub mod onetoone{

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;

  pub fn vecxor(buffer : & Vec<u8>, n :u8) -> Result<Vec<u8>,Box<dyn Error>> {
   //let mut output = Vec::new();
   let mut output = buffer
    .iter()
    .map(|i|{
      let j:u8=i ^ n;
      //println!("int : {} xor  {} = result {} ",i,n,j);
      j
    } )
    .collect();
    Ok(output)
}

   pub fn readd(s : &String) -> Vec<u8>{
    let mut buffer = Vec::new();
    let mut f=File::open(s).expect("Can't Open file");
    f.read_to_end(&mut buffer).expect("Can't Read File");
    buffer
}

   pub fn readc(buffer : & mut Vec<u8>,s : & String){
   let mut f=File::open(s).expect("Can't Open file");
   //let mut buffer = Vec::new();
   f.read_to_end(buffer).expect("Can't Read File");
   }
   pub fn readb(buffer : & mut Vec<u8>) {
   let mut f=File::open("/etc/passwd").expect("Can't Open file");
   //let mut buffer = Vec::new();
   f.read_to_end(buffer).expect("Can't Read File");
   }
   pub fn writeb(buffer : &Vec<u8>){
   let  mut file = File::create("output.bin").expect("Can't Create File");
    file.write_all(buffer).expect("Can't Write File");
   }

    pub fn one(){
     println!("ln my first one");
   }
}
