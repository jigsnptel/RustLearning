use std::collections::HashMap;
fn main(){
   let mut stateCodes = HashMap::new();
   stateCodes.insert("KL","Kerala");
   stateCodes.insert("MH","Maharashtra");
   println!("{:?}",stateCodes);
   println!("size of map is {}",stateCodes.len());

   match stateCodes.get(&"KL") {
    Some(value)=> {
       println!("Value for key KL is {}",value);
    }
    None => {
       println!("nothing found");
    }
 }

 for (key, val) in stateCodes.iter() {
    println!("key: {} val: {}", key, val);
 }
 if stateCodes.contains_key(&"MH") {
    println!("found key");
 }
 stateCodes.remove(&"KL");
 println!("after removing {:?}",stateCodes);


}

