//external packages(crates)
//adding dependencies in cargo.toml file
//cargo run --lib to make the library like npm....
//pakages example / library example
// chrono(date and time for rust)
//to install cargo add chrono and lets use it 
//explore dotenv , uuid, tui,thiserror,sqlx etc.
/*
use chrono :: prelude:: *; //use use chrono::{utc,local};
fn main(){
let utc: DateTime<Utc>=Utc::now();
let local_time: DateTime<Local>=Local::now();
print!("UTC time : {}", utc);
print!("loacl time : {}", local_time);
}
use dotenv::dotenv;
use std::env;
use std::env::VarError;
fn main(){
    dotenv().ok();
    let var :Result<String,VarError>=env::var("KEY");//you can use .unwrap() but this is not good but if yoou know and sure that the value should be there use else it will crash your project
    //  like data base mae use kar sakte hai or use patternmatching try catch etc.
    match var {
        Ok(val)=> println!("The value of KEY is : {}", val),
        Err(e)=> println!("Couldn't read KEY : {}", e),
    }
}

//generics and traits bounds
 //lets start with the question 
 //why this not correct
 //in type script <x> means generics and can have multiple generics like sum<T,U>(a:T,b:U):T{
 //return a
//}
//let x = sum(12345,"sncvjc");
 //function sum<X>(a:X,b:X):X{ //here x can be anything but jo x wo he b rahe ga int to int string to string .
   // return a;
 //}
// now in rust\

use std::ops::Add;
 fn main(){
    print!("{}",sum(1,2));
 }
 fn sum <T :Add <Output = T>> (a:T,b:T)->T{//so add trait bond std::ops::Add<Output = T>
    return a+b; //a+b nai ho sakta kyu ki + hai 
 }
// we can also use or import add like use std::ops::Add;
//which means we are restricting t to use ADD trait.
//if you want to print generic variable then generic variable should bound / should implement the generic trait that is 
//std::fmt::Display
fn main(){
    print_variable(1);
    print_variable(1.0);
    print_variable(String::from("adoityaaaaaaa"));
}
fn print_variable<T : std::fmt::Display>(a:T){
    println!("{}",a);
}

//generics over struct
use std::ops::Mul;
struct Rect<T> {
    width: T,
    height: T,
}

impl<T> Rect<T> // why two <T> because first one is for struct and second one is for impl block first time use i am using after this so we add <T> before rect
where
    T: Mul<Output = T> + Clone,//not have to take ownership useing &self that why clone // clone is also trait so have to bound it
{
    fn area(&self) -> T {
        
//f64 * f64 → f64 ✓
//i32 * i32 → i32 ✓
//String * String ✗ (not allowed, so the trait prevents misuse)
//we have to bound the T with Mul trait
        self.width.clone() * self.height.clone()
    }
}

fn main() {
    let r = Rect {
        width: 10.0,
        height: 10.0,
    };

    println!("{}", r.area());
}
    //if want not to use generic then make 2 different struct for floting and u32 and go on 
*/
//generics over enum