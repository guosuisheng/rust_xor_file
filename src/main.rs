mod zyguo;
use std::env;


fn main() {
 let mut list_of_numbers =  vec![1, 2, 3,0x98,0x66];
 list_of_numbers.push(0xff);
 let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    .map(|i|{
      let j:i32=i ^ 0xf;
      println!("int : {} xor oxf {}",i,j);
      j.to_string()+" Output" 
    } )
    .collect();
    println!("{:?}", list_of_strings);
    println!("Hello, world!");

     let x = String::from("hello");
     let _y = &x;
     let z = &x;
     let closure = || {
       println!("{:?}", z);
       println!("{:?} waht", z);  
         
     };
     let y2 = &x;
     println!("{:?}", y2);
     closure();
     zyguo::onetoone::one();

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
