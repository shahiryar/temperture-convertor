use std::io;

fn main() {

    //TODO: Have the choice returned as an enum to improve consistency and reuseability
    loop{
        let choice = get_choice();
        match choice{
            1=>celsius_2_fahrenheit(),
            2=>fahrenheit_2_celsius(),
            0=>{println!("Exiting the programme.");
        break;},
        _=>println!("\nChoice out of bound! \nOnly enter a choice from the menu above"),
    }
    }
}

fn show_choice(){
    println!("Temperature Convertor");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");
    println!("0. Exit");
    println!("\nEnter your choice (1,2, or 0) : ");
}

fn get_choice()->u8{
    
    let mut choice = String::new();
    show_choice();
    io::stdin()
        .read_line(&mut choice)
        .expect("Choice not read");
    let choice:u8 = choice.trim().parse().expect("Choice Number could not be parsed from the input");
    return choice;
}

fn celsius_2_fahrenheit(){
    let mut temp_cel = String::new();
    println!("\nEnter Temperature in Celsius : ");
    io::stdin()
        .read_line(&mut temp_cel)
        .expect("Line not read!");

    let temp_cel:i32 = temp_cel.trim().parse().expect("Could Not Parse a number from the input");
    let temp_fer = (temp_cel*(9/5))+32;
    println!("Temperature in Fahrenheit : {temp_fer} ");
}


fn fahrenheit_2_celsius(){
    let mut temp_fer = String::new();
    println!("Enter Temperature in Fahrenheit : ");
    io::stdin()
        .read_line(&mut temp_fer)
        .expect("Line not read!");

    let temp_fer:i32 = temp_fer.trim().parse().expect("Could not Parse a number from the input");
    let temp_cel = (temp_fer-32)/(9/5);
    println!("Temp in Celsius : {temp_cel}");
}
