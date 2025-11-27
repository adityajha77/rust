// rather trasferring the ownership to 
//memory cleanup happen when goes out of scope
/* 
fn main(){
    let str: String=String::from("aditya jha");
    let len:usize=get_len(str);
    println!("{}",len); 

}
fn get_len(s2:String) -> usize{
    return s2.len();
}
//now the owner is the s2 not the str but indirect you cahnged the owner
//because str is out of scope at 8 line iske baad nai karega point
//one way is 
//string ko bhi return kar dho like bandi thi fir gai fir aai 
//return(s2.len(),s2) humlog s2 ko bhi return kar diye  but yeah ugly hai
// second way borrowing
//give a reference of value or address of the value use & like for example
fn main(){
    let str=String::from("aditya jha");
     let len:usize=get_len(&str);
    println!("name = {} length is  {}",str,len);
}
fn get_len(str:&String) -> usize{
    let len=str.len();
    return len;
}
// here we borrowing the value aditya jha but borrowing has a rule 
// there can be only one muttable reference and many immutabble refrence
//default all immutable lets see example

fn main(){
    let s1:String=String::from("aditya jha ");
    let s2=&s1;
    let s3=&s1;
    let s4=&s1;
    println!("{} {} {} {}",s1,s2,s3,s4)
}
//multiple immutable example

//one mutabble example
fn main(){
    let mut str:String=String::from("aditya jha ");
    let s1=&mut str;
    let s2=&str;
    println!("{} {}",s1,s2);//error because immutable is using and there is only one muttable
}
//now tricky question is 
fn main(){
    let mut str:String=String::from("aditya jha ");
    let s1=&mut str;
    s1.push_str("hello"); //mutable life ends here
    let s2=&str;
    println!("{}",s2);//one immutable can be there because the mutable life ends in line 54
}
// error will be thrown when one mut and one immut will be there as we seen above

//strcts
struct Rect{
    width:f32,
    height:f32,
}
fn main(){
    let r:Rect =Rect{
        width:10.0,
        height:20.0
    };
    println!("{} {}",r.width,r.height);
}
//struct is used to structure the data.
struct Rect{
    width:f32,
    height:f32,
}
//implementing 
impl Rect{
    fn area(&self) -> f32 {
        return self.width * self.height;
        //first argument is &self if not then it behaves as static functrion
        //if we use &self thenm it is an member function
    }
    fn perimeter(&self) -> f32{
        return 2.0*(self.width + self.height);
    }
    fn print_something(){
        println!("static function");
    }
}
fn main(){
    let r:Rect =Rect{
        width:10.0,
        height:20.0
    };
    println!("{}",r.area());
    //r.print_something is incorreect not belongs to rect but we can use as 
    Rect::print_something();
    println!("{}",r.perimeter());
}

//how to import use rect::Rect; make evry thing pub to other files can access
//pub mod rect

//enums
//define type by enumerating its possible variants 
//means wo sab mae se koi value ho ga let see
enum Direction{
    North,
    South,
    East,
    West
}
//pseudo code
fn main(){
    let direction:Direction=Direction::North;
    steer(dir:direction);
}
fn steer(dir:Direction){
    if(dir==Direction::North){
        println!("you are moving to north");
    }
}
    //explain and use after some time 

//pattern matching

enum Direction {
    North,
    East,
    West,
    South,
}

fn steer(dir: Direction) {
    match dir {
        Direction::North => println!("north direction"),
        Direction::East => println!("east direction"),
        _ => println!("some other direction just runnnn"),   // DEFAULT
    }
}

fn main() {
    let direction = Direction::West;
    steer(direction);
}


| Symbol | Meaning                               | Example                            |
| ------ | ------------------------------------- | ---------------------------------- |
| `:`    | describes the **type**                | `x: i32`, `dir: Direction`         |
| `::`   | accesses a **member inside the type** | `Direction::North`, `String::from` |


//enums with value

enum Shape{
    Square(f32),//means square has one same sides so we put enum with value
    Circle(f32),
    Rectangle(f32,f32)
}
fn main(){
    let s:Shape=Shape::Square(10.0);
    let c:Shape=Shape::Circle(10.0);
    let r:Shape=Shape::Rectangle(10.0,20.0);
    println!("area is => {}",area_find(&s));
    println!("perimeter is => {}",perimeter_find(&s));
    println!("area is => {}",area_find(&c));
    println!("perimeter is => {}",perimeter_find(&c));
    println!("area is => {}",area_find(&r));
    println!("perimeter is => {}",perimeter_find(&r));
}
//writre a function that takes shape as an input and prints its area 
//write one its print its perimeter as well
fn area_find(x:&Shape) -> f32{
    match x{
        Shape::Circle(r) =>3.14*r,
        Shape::Rectangle(w,h)=>w*h,
        Shape::Square(s)=>s*s,
    }
}
fn perimeter_find(x:&Shape) ->f32{
    match x{
        Shape::Circle(r)=>2.0*3.14*r,
        Shape::Rectangle(w,h)=>2.0*(w+h),
        Shape::Square(s)=>4.0*s,
    }
}
//remember Left side of => must be a pattern, not a value.Right side of => must be code, not a value requiring f32
//in this code we lean about mix of borrowing and enum with values
if ; not use then by default it think that the value you want to return....
we can also use impl shape and inside that we can use are (&self).....

//error Handling 
//in js we usee try catch but rust see below

enum Result{
    Ok(String),
    Err(String),
}

    enum Result<T, E> {
    Ok(T),
    Err(E),
}
//but generally it not look like this it look like this 
enum Result<T,E>{ //uses generics study after some time
    Ok(T),
    Err(E),
}
fn main(){
    let contents : 
    match contents{
        ok(contents)=>
        Err(e)=>
    }
}
    //reading file we use library use std::fs

//option enum use to prevent nullability which is millio dollar mistake rust dont have null
//rust uses option enum 
//if nothing found in that content or function it return option that is  Option<u32>
enum Option{
    Some(u32),
    None
}// actually rust have generics enum
pub enum Option<T> {
    None,
    Some(T),
}

// work through the example
fn find_first_a(s: String) -> Option<i32> {
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}

fn main() {
    let my_string = String::from("raman");
    match find_first_a(my_string) {
        Some(index) => println!("The letter 'a' is found at index: {}", index),
        None => println!("The letter 'a' is not found in the string."),
    }
}
*/
