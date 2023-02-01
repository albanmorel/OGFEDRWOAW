use colored::Colorize;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::cmp::max;
use std::vec::Vec;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct CreatureCard {
    name: String,
    power: u32,
    endurance: u32,
    mana_cost: u32,
    tapped: bool,
}
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct LandCard {
    name: String,
    mana_value: u32,
    tapped: bool,
}

fn new_creature_card() -> CreatureCard {
    CreatureCard {
        name: "Creature_Card_Name".to_string(),
        power: 3,
        endurance: 3,
        mana_cost: 1,
        tapped: false,
    }
}

fn new_land_card() -> LandCard {
    LandCard {
        name: "Land_Card_Name".to_string(),
        mana_value: 1,
        tapped: false,
    }
}
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
enum Cards {
    Creature(CreatureCard),
    Land(LandCard),
}

#[derive(Clone, Debug)]
enum GameFormat {
    Standard = 60,
    Limited = 40,
    Commander = 100,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
enum PlayerType {
    Goldfish,
    Human {
        deck: Vec<Cards>,
        hand: Vec<Cards>,
        board: Vec<Cards>,
    },
}
#[derive(Hash, Eq, PartialEq, Debug)]
struct Player {
    hp: u32,
    turn_order: u32,
    player_type: PlayerType
}

impl PlayerType {
    pub fn create_deck(&self, format: u32 ) -> PlayerType {
        match self {
            PlayerType::Human => {
                PlayerType::Human{
                    deck: create_deck(format),
                    hand: Vec::<Cards>::new(),
                    board: Vec::<Cards>::new(),
                }
            },
            _ => {
                PlayerType::Goldfish
            }
        }
    }

    pub fn shuffle_player_deck(&mut self) {
        match self {
            PlayerType::Human => {
                self.deck.shuffle(&mut thread_rng());
            }
            _ => {}
        };
    }
    
    pub fn draw_player_card(player: &mut Player) -> () {
        match self {
            PlayerType::Human => {
                self.hand.push(self.deck.pop().unwrap());  /*TODO: remplacer uwrap par faire perdre la game*/
            }
            _ => {}
        };
}

    
}

fn new_player(player_type: PlayerType, format: u32, turn_order: u32) -> Player {
    Player {
        hp: 20,
        turn_order,
        player_type :player_type.create_deck(format),
    }
}
/*
fn draw_creature(creature_power: u32, creature_endurance: u32, creature_cost: u32){
    print!("
     _________
    | Creat |{creature_cost}|
    |         |
    |         |
    |     {creature_power}/{creature_endurance} |
    |_________|");

}

fn draw_land(land_mana_value: u32){

    println!("
    _________
   |  Land   |
   |         |
   |   |{land_mana_value}|   |
   |         |
   |_________|");
} */

fn create_players_and_decks(
    number_of_human_player: u32,
    number_of_goldfish_player: u32,
    format: usize,
) -> Vec<Player> {
    let mut players_list = Vec::<Player>::new();
    let mut turn_order: u32 = 1;
    for i in 1..number_of_human_player + number_of_goldfish_player + 1 {
        if i <= number_of_human_player {
            players_list.push(new_player(PlayerType::Human, format, turn_order));
            turn_order += 1;
        }
        else {
            players_list.push(new_player(PlayerType::Goldfish, format, turn_order));
            turn_order += 1;
        }
    }
    players_list
}

fn choose_format() -> GameFormat {
    println!("Yoo c'est quoi le format ?");
    println!(
        "1. Standards (60 cartes) - Defaut (ça veut dire que tout les inputs font ça sauf 2 et 3)"
    );
    println!("2. Limited (40 cartes)");
    println!("3. Commander (100 cartes) ");

    let mut user_input: String = String::new();
    std::io::stdin().read_line(&mut user_input).unwrap();

    let choosed_format = match user_input.trim().parse::<u32>() {
        Ok(user_input) if user_input == 2 => GameFormat::Limited,
        Ok(user_input) if user_input == 3 => GameFormat::Commander,
        Ok(_user_input) => GameFormat::Standard,
        Err(_e) => GameFormat::Standard,
    };

    choosed_format
}

fn create_deck(deck_size: u32) -> Vec<Cards> {
    let mut deck = Vec::<Cards>::new();
    let creature = new_creature_card();
    let land = new_land_card();

    while deck.len() < deck_size {
        match rand::thread_rng().gen_range(0..2) {
            0 => {
                deck.push(Cards::Creature(creature.clone()));
            }
            1 => {
                deck.push(Cards::Land(land.clone()));
            }
            _ => {
                println!("Sauce");
            }
        };
    }
    deck
}
fn shuffle_decks(players_list: &mut Vec<Player>) -> () {
    for player in players_list {
        player.player_type.shuffle_player_deck();
    };
}
fn draw_hands(players_list: &mut Vec<Player>) -> () {
    for player in players_list {
        for _i in 0..7 {
            draw_card(player)
        }
    }
}

fn get_nb_of_human_player() -> u32 {
    loop {
        println!("Yooo ya combien de human qui jouent ? (Maximum 4 joueurs)");
        let mut user_input: String = String::new();
        std::io::stdin().read_line(&mut user_input).unwrap();

        let number_of_human_player_input = match user_input.trim().parse::<u32>() {
            Ok(user_input) if user_input <= 4 => user_input,
            Ok(_user_input) => {
                println!(
                    "{}",
                    ("Yoo very desoled mais c'est 4 joueurs maximum :/")
                        .italic()
                        .red()
                );
                continue;
            }
            Err(_e) => {
                println!("{}", ("Yooo g pa kompri").italic().red());
                continue;
            }
        };
        return number_of_human_player_input;
    }
}
fn get_nb_of_goldfish_player(number_of_human_player: u32) -> u32 {
    let mut number_of_goldfish_player_input: u32 = 0;
    if number_of_human_player < 4 {
        loop {
            println!("Yooo ya combien de goldfish qui jouent ?");
            let mut user_input: String = String::new();
            std::io::stdin().read_line(&mut user_input).unwrap();

            number_of_goldfish_player_input = match user_input.trim().parse::<u32>() {
                Ok(user_input) if user_input <= 4 - number_of_human_player => user_input,
                Ok(_user_input) => {
                    println!("{}", format!("Yoo very desoled mais c'est 4 joueurs maximum et il y a déjà {number_of_human_player} joueurs human").italic().red());
                    continue;
                }
                Err(_e) => {
                    println!("Yooo g pa kompri");
                    continue;
                }
            };
            break;
        }
    }
    number_of_goldfish_player_input
}

fn display_players_decks(players_list: &Vec<Player>) -> () {
    for player in players_list {
        println!(
            "{}",
            format!("\n\nDeck du joueur {:?}\n", player.player_type)
                .yellow()
                .bold()
        );
        for card in &player.player_type.deck {
            match card {
                Cards::Land(card) => {
                    println!("{}", format!("{:?}", card).green())
                }
                Cards::Creature(card) => {
                    println!("{}", format!("{:?}", card).cyan())
                }
            }
        }
    }
}

fn display_player_hand(player: &Player::PlayerType::Human) -> () {
    println!("{}", format!("\n\nMain actuelle :\n").yellow().bold());

    let mut i: u32 = 0;
    for card in &player.hand {
        i += 1;
        match card {
            Cards::Land(card) => {
                print!("{}", format!("{i}. "));
                println!("{}", format!("{:?}", card).green())
            }
            Cards::Creature(card) => {
                print!("{}", format!("{i}. "));
                println!("{}", format!("{:?}", card).cyan())
            }
        }
    }
}

fn display_player_bord(player: &Player::PlayerType::Human) -> () {
    println!("{}", format!("\n\n Board du joueur :\n").yellow().bold());

    for card in &player.board {
        match card {
            Cards::Land(card) => {
                println!("{}", format!("{:?}", card).green())
            }
            Cards::Creature(card) => {
                println!("{}", format!("{:?}", card).cyan())
            }
        }
    }
}

fn initialize_game_and_players() -> Vec<Player> {
    let format: GameFormat = choose_format();

    let number_of_human_player: u32 = get_nb_of_human_player();
    let number_of_goldfish_player: u32 = get_nb_of_goldfish_player(number_of_human_player);

    let players_list: Vec<Player> = create_players_and_decks(
        number_of_human_player,
        number_of_goldfish_player,
        format as u32,
    );
    players_list
}

fn play(players_list: &mut Vec<Player>) -> () {
    let mut turn_played: u32 = 0;
    let number_of_player: u32 = players_list.len() as u32;

    println!("{:?}", players_list.len());
    for player in players_list {
        println!("pass");
        println!("player turn_order: {}", player.turn_order);
        println!("turn_played : {}", turn_played);
        println!("modulo: {}", (turn_played % number_of_player) + 1);

        if player.turn_order == (turn_played % number_of_player) + 1 {
            println!("pass1");
            play_player_turn(player);
            turn_played += 1;
        }
    }
}

fn play_player_turn(player: &mut Player) -> () {
    match player.player_type {
        PlayerType::Human => {
            play_player_main_phase(player);
            play_player_combat_phase(player);
            //play_player_main_phase(player);
        }
        else {
            play_goldfish_turn(player);
        }
    }
}

fn play_player_main_phase(player: &mut Player::PlayerType::Human) -> () {
    let mut land_played: bool = false;
    loop {
        display_player_hand(player);
        println!(
            "Yooo tu veux jouer quoi ? (Appuie sur n'importe quelle touche pour passer ton tour)"
        );
        let mut user_input: String = String::new();
        std::io::stdin().read_line(&mut user_input).unwrap();

        match user_input.trim().parse::<usize>() {
            Ok(choosed_card_index) if choosed_card_index <= player.hand.len() => {
                if can_player_play_card(player, choosed_card_index - 1, &mut land_played) {
                    land_played = play_card(player, choosed_card_index - 1, &land_played);

                    display_player_bord(player);
                    continue;
                };
                continue;
            }
            Ok(choosed_card_index) => {
                println!("Yooo cette carte existe pas");
                continue;
            }
            Err(_e) => {
                println!("Passe le tour");
                break;
            }
        };
    }
}

fn play_player_combat_phase(player: &mut Player) -> () {}

fn can_player_play_card(player: &mut Player, card_index: usize, land_played: &bool) -> bool {
    println!("land played : {:?}", land_played);

    match &player.hand[card_index] {
        Cards::Land(_card) if *land_played => {
            println!("Yooo déso mais t'as déjà joué un land ce tour ci");
            return false;
        }
        Cards::Land(_card) => {
            return true;
        }
        Cards::Creature(_card) => {
            if check_for_mana(player, card_index) {
                return true;
            } else {
                println!("Yooo déso mais t'as pas le mana pour cast ça");
                return false;
            }
        }
    };
    //display_player_bord(player);
}
fn play_card(player: &mut Player, card_index: usize, land_played: &bool) -> bool {
    match &player.hand[card_index] {
        Cards::Land(card) => {
            player.board.push(player.hand.remove(card_index));
            return true;
        }
        Cards::Creature(card) => {
            println!("{}", card.mana_cost);
            for i in 0..card.mana_cost {
                tap_a_mana_card(player);
            }
            player.board.push(player.hand.remove(card_index));
            return *land_played;
        }
    }
}

fn check_for_mana(player: &mut Player, card_index: usize) -> bool {
    get_spell_cost(&player.hand[card_index]) <= get_untapped_mana(player)
}

fn get_spell_cost(card: &Cards) -> u32 {
    match card {
        Cards::Land(card) => 0,
        Cards::Creature(card) => card.mana_cost,
    }
}

fn get_untapped_mana(player: &Player) -> u32 {
    let mut untapped_mana: u32 = 0;
    for card in &player.board {
        match card {
            Cards::Land(card) if card.tapped == false => {
                untapped_mana += 1;
            }
            _ => {}
        }
    }
    untapped_mana
}

fn tap_card(card: &mut Cards) -> () {
    println!("pass");
    println!("{:?}", card);
    match card {
        Cards::Land(card) => {
            println!("pass");
            card.tapped = true;
        }
        Cards::Creature(card) => {
            card.tapped = true;
        }
    }
}

fn tap_a_mana_card(player: &mut Player) -> () {
    println!("je passe");
    for card in &mut player.board {
        println!("woooouu");
        match card {
            Cards::Land(_card) => {
                tap_card(card);
                break;
            }
            _ => {}
        }
    }
}

fn main() {
    let mut players_list: Vec<Player> = initialize_game_and_players();
    println!("{:?}",players_list);
    //display_players_decks(&players_list);
    shuffle_decks(&mut players_list);
    draw_hands(&mut players_list);
    //display_player_hand(&players_list[0]);
    //println!("{}", &players_list[0].deck.len());
    play(&mut players_list);
}
//Accepter un joueur
//generer un deck de carte
//montrer deck de carte au joueur
//generer un adversaire goldfish
//lancer une partie
//draw une main au joueur
//#### phase principale
//joueur selectionne carte ou skip phase
//vérifier si la carte est valide (et si le cout peut être payé)
//#### phase combat
//Si le joueur à des créatures, il déclare attanquants
//résolution de l'attaque
//### seconde phase principale
//refaire phase principale #1 avec nouveau board state
//Tour du goldfish
//Il déclare attanquant aléatoire pour taper le joueur
//Phase de bloquages
//fin du tour retour au début
