use std::io;

fn main() {
    println!("Type d for degree to farenheight and f for the other way round");
    let mut deg_or_frh = String::new();
    
    io::stdin().read_line(&mut deg_or_frh).expect("Failed to read the line.");

    println!("Enter the value of temprature:");
    let mut input_value = String::new();
        
    io::stdin().read_line(&mut input_value).expect("Failed to read the value.");

    let input_value: f32 = match input_value.trim().parse(){
        Ok(num) => num,
        _ => panic!(),

    };

    if deg_or_frh.trim() == "d"{
        let far = input_value * 9.0/5.0 + 32.0;
        println!("The temprature in farenheight is :{far}");
    }else if deg_or_frh.trim()== "f"{
        let deg = (input_value - 32.0) * 5.0/9.0;
        println!("The temprature in degree celcius is :{deg}");
    }else{
        println!("There was some error. Pls try again.");
    }
}
