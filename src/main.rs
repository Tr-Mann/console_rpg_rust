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
#[derive(Clone)]
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
    combat(mainchar, random_enemy(initialize_enemies()));
}

fn character_creation() -> Player{
    let mut input: String = String::new();
    println!("What's your name"); 
    io::stdin().read_line(&mut input).expect("Not a real name bub");
    let mut mainchar = Player{
    name: input,
    health: 15,
    damage: 10,
    };
    mainchar
}

//Initialize enemies
fn initialize_enemies() -> [Enemy; 3]{
    //Find out how to use an array of this instead
    let mut goblin = Enemy{
    enemy_type: String::from("Goblin"),
    damage: 4,
    health: 15,
    };
    
    let mut slime = Enemy{
        enemy_type: String::from("Slime"),
        damage: 2,
        health: 10,
    };

    let mut grung = Enemy{
        enemy_type: String::from("grung"),
        damage: 5,
        health: 10,
    };
    //This line creates an array with the enemy data stored 
    let enemy_array:[Enemy; 3] = (goblin, slime, grung).into();
    enemy_array
}

fn random_enemy(enemy_array: [Enemy; 3]) -> Enemy{
    //Generate random number
    let mut rng = rand::thread_rng();
    let rng = rng.gen_range(0..2);
    //Get enemy from that random index and return
    let chosen: Enemy;
    chosen = enemy_array[rng].clone();
    chosen
}

fn combat(mut mainchar: Player, mut opp: Enemy){
    let mut rng = rand::thread_rng();
    let mut fighting: bool = true;
    while fighting{
        println!("You find a wild {}\n 1.Attack\n 2.Heal\n", opp.enemy_type);
        //Create container and get store user's input
        let input = get_input(2);

        if input == 1 {
            opp.health -= rng.gen_range(0..mainchar.damage); 
            println!("You hit the {} for {} damage it has {} health remaining", opp.enemy_type,"some", opp.health);
        }
        else if input == 2{
            mainchar.health += 2;
        }
        if opp.health <= 0 || mainchar.health <= 0{
            fighting = false;
        }
        //enemy turn next
    }
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



