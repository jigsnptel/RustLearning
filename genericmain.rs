struct Data<T> {
    value:T,
 }

 fn print_pro<T:Display>(t:T){
    println!("Inside print_pro generic function:");
    println!("{}",t);
 }

 use std::fmt::Display;
 fn main() {
    //generic type of i32
    let t:Data<i32> = Data{value:350};
    println!("value is :{} ",t.value);
    //generic type of String
    let t2:Data<String> = Data{value:"Tom".to_string()};
    println!("value is :{} ",t2.value);

    /*The example defines a generic function that displays a parameter passed to it. 
    The parameter can be of any type. The parameter’s type should implement the 
    Display trait so that its value can be printed by the println! macro. */
   print_pro(10 as u8);
   print_pro(20 as u16);
   print_pro("Hello world");
}





 
