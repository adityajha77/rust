//fn main() {
  //  println!("Hello, world!"); //macro
//}

/*fn main(){
    let ans: u32 =sum(10,20); //use u32 u64 u128 to assign how  and what yoyu want 
    // we can use i32 i64 i 128 also but we use to represent negative if 0 positive if 1 negative if 32 then 31 considered and last bit is for sign
    println!("{}",ans); //directly we cant write ans in main like this -> println!(ans);
}

fn sum(a:u32, b:u32) -> u32 { //u means unsigned integer
    return a+b;
}

//even odd check code
fn main(){
    println!("{}",check(10));
}
/*fn check(a:u32){
    if a%2==0 {println!("even")}
    if a%2==1 {println!("odd")};
}*/
fn check(a:u32) -> bool{
return a%2==0;
}

//memory management in rust is fast and safe no need of manual memeory mangement
fn main(){
    let name:String=String::from("adityaJHA");
    println!("whats you name -> {}",name); 
} 

fn main(){
    let v:Vec<u32>=vec![1,2,3,4,5];
   // println!("{}",v); // help: the trait `std::fmt::Display` is not implemented for `Vec<u32>`
   //= note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead 
println!("{:?}",v);
}

fn main(){
    for i in 1u32..10{
        println!("{}",i);
    }
}

//muttable vs immutable (by default it is immutable immutable means cant  change the value muttable means can change the value )
fn main() {
    let mut x: i32 = 1;  // mutable variable
    println!("{}", x);  // prints 1
    x = 2;               // update value
    println!("{}", x);  // prints 2
}
fn main(){
    let mut name :String = String::from("aditya");
    println!("{}",name);
    name.push_str("jha");
    println!("{}",name);
}

//rust protect the code with owenership concept
//stack ka size badha nai sakte program run ho raha tho but heap kar sakta hai
//heap mae store ho ga aur stack mae ek pointer store ho ga jo point kare ga heap ko.
// integer wagera mae nai hota bas string mae hota
//heap kab hatega memory mangement ? jab kaam ho jaiga auska hat jai ga aur kon kab hatega wo ownership management dekhe ga
//in c++ we use dealloc to deallocate the sapace but in rust we dont jaise he out of scope hoga } khatam heap se value gayab
//par ab dikat kaha aai ga jab auska use kopi aur aadmi kar raha 
// x=5
//y=x ab kya hoga ki owner ban jaiga y kyu ki dangling pointing nai ho ho aur mix aur drop nai ho 
fn main(){
    let mut name :String = String::from("aditya");
    let name2:String=name;
    println!("{}",name);
    name.push_str("jha");
    println!("{}",name);//yaha pe error de dega kyu ki name ab nai use ho ga ab name 2 owner ban gaya hai 
}
    prevent double free error and dangel free error 
    //we can use clone() also to prevent this   
    // so at last one owner at a time when owner goes out of scope the value will dropped .
*/
fn main(){
    let name:String=String::from("ADITYA");//we can use &str
    let len=get_len(name);
    println!("{}",len);
    println!("{}",name);
}
fn get_len(s:String) -> usize{
    return s.len();
}
//actually yaha pe owner kon hai s naki name kyu ki owner ship hum log de diye line 82 pe
// aur agar kar an he hai tho borrowing sikho fir aao meet in next class 11/27/2025.
//trust for rust in youtube video serde