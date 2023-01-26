use bevy::prelude::*;
use bevy::app::App;
use serde_json::{Value};

#[derive(Component)]
struct Game;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Population(u64);

#[derive(Component)]
struct Price(f64);

#[derive(Component)]
struct Rewards(f64);

#[derive(Component)]
struct Currency(f64);

#[derive(Component)]
struct Subscription(bool);

#[derive(Component)]
struct DifficultyBase(f64);

#[derive(Component)]
struct DifficultyGrowth(f64);

#[derive(Component)]
struct DifficultyTicks(f64);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Workers(u16);

#[derive(Component)]
struct RebornPoints(f64);

#[derive(Component)]
struct IdleTokens(f64);








fn create_player(mut commands: Commands) {
    commands.spawn((Player, RebornPoints(0.0), IdleTokens(0.0), Currency(0.0), Workers(0), DifficultyBase(1.0), DifficultyGrowth(0.1), DifficultyTicks(0.0)));
}

fn read_games_json() -> serde_json::Value {
    // Read src/games.json
    /* {
        "Name": {
            "price": 0.0,
            "subscription": false,
            "difficulty_base": 1,
            "difficulty_growth": 0.0,
            "difficulty_ticks": 0.0,
            "rewards": 1.0,
            "workers": 0,
            "currency": 0.0,
            "population": 0
        }
    }*/
    let games_json = std::fs::read_to_string("src/games.json").expect("Unable to read games.json");
    let games: Value = serde_json::from_str(&games_json).expect("Unable to parse games.json");
    games


}
// Add Game entity to the world
fn add_games(mut commands: Commands) {
    let games_list: Value = read_games_json();

    for (game_name, game_data) in games_list.as_object().unwrap() {
        commands.spawn((Game, 
            Name(game_name.to_string()),
            Price(game_data["price"].as_f64().unwrap()),
            Subscription(game_data["subscription"].as_bool().unwrap()),
            DifficultyBase(game_data["difficulty_base"].as_f64().unwrap()),
            DifficultyGrowth(game_data["difficulty_growth"].as_f64().unwrap()),
            DifficultyTicks(game_data["difficulty_ticks"].as_f64().unwrap()),
            Rewards(game_data["rewards"].as_f64().unwrap()),
            Workers(game_data["workers"].as_u64().unwrap() as u16),
            Currency(game_data["currency"].as_f64().unwrap()),
            Population(game_data["population"].as_u64().unwrap())
        ));
    }

    println!("Games added to the world");
    
}

// Timer to print games
#[derive(Resource)]
struct PrintTimer(Timer);

// Query all games and print them
fn print_games(time: Res<Time>, mut timer: ResMut<PrintTimer>, query: Query<(&Name, &Price, &Subscription, &DifficultyBase, &DifficultyGrowth, &DifficultyTicks, &Rewards, &Workers, &Currency, &Population)>) {
    if timer.0.tick(time.delta()).just_finished() {
        for (name, price, subscription, difficulty_base, difficulty_growth, difficulty_ticks, rewards, workers,  currency, population) in query.iter() {
            println!("{}", name.0);
            println!("\tPrice: \t\t{}", price.0);
            println!("\tSubscription: {}", subscription.0);
            println!("\tDiff_base: \t{}", difficulty_base.0);
            println!("\tDiff_grow: \t{}", difficulty_growth.0);
            println!("\tDiff_ticks: \t{}", difficulty_ticks.0);
            println!("\tRewards: \t{}", rewards.0);
            println!("\tWorkers: \t{}", workers.0);
            println!("\tCurrency: \t{}", currency.0);
            println!("\tPopulation: \t{}", population.0);
        }
    }
}


// Timer for game tick
#[derive(Resource)]
struct GameTimer(Timer);

fn game_tick(time: Res<Time>, mut timer: ResMut<GameTimer>, mut query: Query<(&Name, &mut DifficultyTicks, &Rewards, &mut Workers, &mut Currency)>) {
    if timer.0.tick(time.delta()).just_finished() {
        for (_, mut difficulty_ticks, rewards, workers, mut currency) in query.iter_mut() {
            difficulty_ticks.0 += 1.0;
            currency.0 += rewards.0 * workers.0 as f64;
        }       

        // Add worker to pwn adventure 3
        assign_worker(query, "Pwn Adventure 3".to_string());
    }

}

// Assign worker to game based on name
fn assign_worker(mut query: Query<(&Name, &mut DifficultyTicks, &Rewards, &mut Workers, &mut Currency)>, game_name: String) {
    for (_,  _, _, mut workers, _) in query.iter_mut() {
        if game_name == "Pwn Adventure 3" {
            workers.0 += 1;
        }
    }
}


fn main() {
    App::new()
        .insert_resource(PrintTimer(Timer::from_seconds(3.0, TimerMode::Repeating)))
        .insert_resource(GameTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(create_player)
        .add_startup_system(add_games)
        .add_system(game_tick)
        .add_system(print_games)
        .run();
}
