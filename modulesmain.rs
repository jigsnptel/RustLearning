pub mod movies {
    pub fn play(name:String) {
       println!("Playing movie {}",name);
    }
 }
 use movies::play;

 fn main(){
    //movies::play("Herold and Kumar".to_string());
    play("Herold and Kumar".to_string());
 }
 
