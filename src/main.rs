use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsString;
use std::io;
use std::process::Command;

use itertools::Itertools;
use os_switcher::*;
use regex::Regex;

pub mod os_switcher;

fn main(){
    let mut os_switcher: Box<dyn OSSwitcher> = Box::new(BCDOSSwitcher::new());

    match get_user_os_choice(&os_switcher.get_os_options().unwrap())
    {
        Ok(choice) => {
            println!("Chosen OS: {}", choice);
            os_switcher.switch_os(choice).unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("error: unable to read user input");
        },
        Err(err) => println!("Error: {}", err)
    }
}

fn get_user_os_choice(os_selections: &HashMap<u32, OS>) -> Result<u32, Box<dyn Error>>{
    for os_choice in os_selections.keys().sorted() {
        println!("[{}] {:?}", os_choice, os_selections[os_choice]);
    }

    println!("Please choose your OS that you want to switch to: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    //println!("{}", input.trim().parse::<u32>().unwrap());
    let mut choice: u32 = 0;
    match input.trim().parse::<u32>() {
       Ok(user_choice) => choice = user_choice,
        Err(err) => Err("The choice is invalid")?
    }

    return if os_selections.contains_key(&choice)
    {
        Ok(choice)
    } else {
        Err("The choice is invalid")?
    }
}