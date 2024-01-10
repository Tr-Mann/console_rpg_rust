//========================
//Author: Tristan Mann
//Date January 9th 2024
//========================

use std::io;
//Struct used for the player's stats 
struct Player{
    name: String,
    damage: i32,
    health: i32,
}
//Enemy data struct
struct Enemies{
    name: String,
    damage: i32,
    health: i32,
}
fn main() {
    let mut input: i32;
    let mut is_running: bool = true;

    //Main game loop
    while is_running{
        main_menu();
        input = get_input(2);

        if input == 1{
            //Main game loop
            game();
        }
        else if input == 2{
            //exit function
            is_running = exit();
        }
    }
}

//used to display the main menu of the game
fn main_menu(){
    println!("G.L.O.O.P: An adventure!\n 1.Play\n 2.Exit\n");
}
//used to gather and validate input from user
fn get_input(max:i32) -> i32{
    let mut input = String::new();
    let mut is_valid: bool = false;
    print!("Make a selection:\n ");
    io::stdin().read_line(&mut input).expect("Invalid");
    let input = input.trim().parse::<i32>().expect("Holy moly");
    if input <= max && input > 0 {
        is_valid = true;
    }
    while !is_valid{
        println!("Invalid input try again");
        //Make input a tring again
        let mut input = String::new();
        //get input then convert to i32 again
        io::stdin().read_line(&mut input).expect("Invalid");
        let input = input.trim().parse::<i32>().expect("Holy moly");
        
        if input <= max && input > 0 {
            return input;
        }
    }
    
    input
}

fn game(){
    let mainchar: Player = character_creation();
    println!("Hello {} welcome to your adventure!", mainchar.name);
}

fn character_creation() -> Player{
    let mut input: String = String::new();
    println!("What's your name"); 
    io::stdin().read_line(&mut input).expect("Not a real name bub");
    
    //Creating an instance of a struct and initializing it properly
    let mainchar = Player{
    name: input, 
    health: 15,
    damage:  8,
    };
    mainchar
}

fn exit() -> bool{
    let input: i32;
    println!("Exit?\n 1.Yes\n 2.No");
    input = get_input(2);
    if input == 1{
        return false;
    }
    return true;
}