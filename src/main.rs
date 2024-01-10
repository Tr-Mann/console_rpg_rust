//======================================================================
//Author: Tristan Mann
//Date January 9th 2024
//This projects is being rewritten in rust using LazyVim as code editor
//======================================================================

use rand::Rng;
use std::io;
//Struct used for the player's stats 
struct Player{
    name: String,
    damage: i32,
    health: i32,
}
//Enemy data struct
struct Enemy{
    enemy_type: String,
    damage: i32,
    health: i32,
}
fn main() {
    let mut input: i32;
    let mut is_running: bool = true;

    //Main game loop
    while is_running{
        main_menu();
        initialize_enemies();
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
    let mainchar = Player{
    name: input,
    health: 15,
    damage: 10,
    };
    mainchar
}

//Initialize enemies
fn initialize_enemies() -> [Enemy; 3]{
    //Find out how to use an array of this instead
    let goblin = Enemy{
    enemy_type: String::from("Goblin"),
    damage: 4,
    health: 15,
    };
    
    let slime = Enemy{
        enemy_type: String::from("Slime"),
        damage: 2,
        health: 10,
    };

    let grung = Enemy{
        enemy_type: String::from("grung"),
        damage: 5,
        health: 10,
    };
    //This line creates an array with the goblin, and slime objects stored inside
    //
    let enemy_array:[Enemy; 3] = (goblin, slime, grung).into();
    enemy_array
}

fn random_enemy(enemy_array:[Enemy; 3]) -> Enemy{
    
}

fn combat(mainchar: Player, opp: Enemy){
    
    
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



