mod zyguo;
use std::env;


fn main() {
     let mut n:u8=0xf;
     //let mut s: &String;
     let args: Vec<String> = env::args().collect();
     let nn=args.len();
     println!("My path is {}. {} nn is {}", args[0],args.len(),nn);
     
     let mut s=String::from("Default");
     match nn{
     2 => {
        s=args[1].clone();
        //n=args[1].parse::<u8>().expect("Need a integer");
     },
     3 => {
        s=args[1].clone();
        n=args[2].parse::<u8>().expect("Need a integer");
     },
     _ => {
        s=String::from("Error");
        println!("Help");
     },
     };

     println!("My filename is{}",s);

     let mut buf = Vec::new();
     zyguo::onetoone::readc(&mut buf,&s);


     let buf2=zyguo::onetoone::readd(&s);
     println!("{:?} {:?}",buf[0],buf.len());
     println!("{:?} {:?}",buf2[0],buf2.len());

   let output2=zyguo::onetoone::vecxor(&buf2,n).unwrap();
   zyguo::onetoone::writeb(&output2);
}
