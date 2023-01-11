
    
    fn main() {
        // creating vector using new
        let mut v = Vec::new();
        v.push(20);
        v.push(30);
        v.push(40);
     
        println!("size of vector is :{}",v.len());
        println!("{:?}",v);

        //creating a vector using Vec macro
        let mut v = vec![1,2,3];
        println!("{:?}",v);
        v.remove(1);
        println!("{:?}",v);
        println!("len is {}",v.len());
        if v.contains(&1) {
            println!("found 1");
         }
      
     
     }
     

