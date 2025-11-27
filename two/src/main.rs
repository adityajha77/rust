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
*/
