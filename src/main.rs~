use std::io;

fn main() {

    let mut choice = String::new();
    println!("Temperature Convertor");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");
    println!("\nEnter your choice (1 or 2) : ");
    io::stdin()
        .read_line(&mut choice)
        .expect("Choice not read");
    let choice:u8 = choice.trim().parse().expect("Choice Number could not be parsed from the input");

    if choice == 1{
    
        let mut temp_cel = String::new();
        println!("\nEnter Temperature in Celsius : ");
        io::stdin()
            .read_line(&mut temp_cel)
            .expect("Line not read!");

        let temp_cel:i32 = temp_cel.trim().parse().expect("Could Not Parse a number from the input");
        let temp_fer = (temp_cel*(9/5))+32;
        println!("Temperature in Fahrenheit : {temp_fer} ");
    
    }else if choice == 2{
    
        let mut temp_fer = String::new();
        println!("Enter Temperature in Fahrenheit : ");
        io::stdin()
            .read_line(&mut temp_fer)
            .expect("Line not read!");

        let temp_fer:i32 = temp_fer.trim().parse().expect("Could not Parse a number from the input");
    
        let temp_cel = (temp_fer-32)/(9/5);
        println!("Temp in Celsius : {temp_cel}");
    }

}
