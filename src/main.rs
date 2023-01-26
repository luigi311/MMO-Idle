use bevy::app::App;
use bevy::prelude::*;
use serde_json::Value;

#[derive(Component)]
struct Currency(f64);

#[derive(Component)]
struct DifficultyBase(f64);

#[derive(Component)]
struct DifficultyGrowth(f64);

#[derive(Component)]
struct DifficultyTicks(f64);

#[derive(Component)]
struct Workers(u64);

#[derive(Component)]
struct WorkersCap(u64);

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
struct Subscription(bool);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct RebornPoints(f64);

#[derive(Component)]
struct IdleTokens(f64);

fn create_player(mut commands: Commands) {
    commands.spawn((
        Player,
        RebornPoints(0.0),
        IdleTokens(0.0),
        Currency(0.0),
        Workers(0),
        WorkersCap(1),
        DifficultyBase(1.0),
        DifficultyGrowth(0.1),
        DifficultyTicks(0.0),
    ));
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
            "workers_cap": 0,
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
        commands.spawn((
            Game,
            Name(game_name.to_string()),
            Price(game_data["price"].as_f64().unwrap()),
            Subscription(game_data["subscription"].as_bool().unwrap()),
            DifficultyBase(game_data["difficulty_base"].as_f64().unwrap()),
            DifficultyGrowth(game_data["difficulty_growth"].as_f64().unwrap()),
            DifficultyTicks(game_data["difficulty_ticks"].as_f64().unwrap()),
            Rewards(game_data["rewards"].as_f64().unwrap()),
            Workers(game_data["workers"].as_u64().unwrap()),
            WorkersCap(game_data["workers_cap"].as_u64().unwrap()),
            Currency(game_data["currency"].as_f64().unwrap()),
            Population(game_data["population"].as_u64().unwrap()),
        ));
    }

    println!("Games added to the world");
}

// Timer to print games
#[derive(Resource)]
struct PrintTimer(Timer);

// Query all games and print them
fn print_all(
    time: Res<Time>,
    mut timer: ResMut<PrintTimer>,
    query: Query<(
        &Currency,
        &Workers,
        &WorkersCap,
        &DifficultyBase,
        &DifficultyGrowth,
        &DifficultyTicks,
        Option<&Game>,
        Option<&Name>,
        Option<&Price>,
        Option<&Rewards>,
        Option<&Subscription>,
        Option<&Population>,
        Option<&Player>,
        Option<&RebornPoints>,
        Option<&IdleTokens>,
    )>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (
            currency,
            workers,
            workers_cap,
            difficulty_base,
            difficulty_growth,
            difficulty_ticks,
            game,
            name,
            price,
            rewards,
            subscription,
            population,
            player,
            reborn_points,
            idle_tokens,
        ) in query.iter()
        {
            if let Some(_game) = game {
                println!("{}", name.unwrap().0);
                println!("\tPrice: \t\t{}", price.unwrap().0);
                println!("\tSubscription: {}", subscription.unwrap().0);
                println!("\tDiff_base: \t{}", difficulty_base.0);
                println!("\tDiff_grow: \t{}", difficulty_growth.0);
                println!("\tDiff_ticks: \t{}", difficulty_ticks.0);
                println!("\tRewards: \t{}", rewards.unwrap().0);
                println!("\tWorkers: \t{}", workers.0);
                println!("\tWorkers_cap: \t{}", workers_cap.0);
                println!("\tCurrency: \t{}", currency.0);
                println!("\tPopulation: \t{}", population.unwrap().0);
            } else if let Some(_player) = player {
                println!("Player");
                println!("\tRebornPoints: \t{}", reborn_points.unwrap().0);
                println!("\tIdleTokens: \t{}", idle_tokens.unwrap().0);
                println!("\tCurrency: \t{}", currency.0);
                println!("\tWorkers: \t{}", workers.0);
                println!("\tWorkers_cap: \t{}", workers_cap.0);
                println!("\tDiff_base: \t{}", difficulty_base.0);
                println!("\tDiff_grow: \t{}", difficulty_growth.0);
                println!("\tDiff_ticks: \t{}", difficulty_ticks.0);
            } else {
                println!("Unknown entity");
            }
        }
    }
}

fn difficulty_tick(
    time: Res<Time>,
    mut query: Query<(
        &mut DifficultyTicks,
        &Workers,
        &WorkersCap,
        Option<&Game>,
        Option<&Player>,
    )>,
) {
    for (mut difficulty_ticks, workers, workers_cap, game, player) in query.iter_mut() {
        if let Some(_game) = game {
            if workers.0 > 0 {
                // Set tick to time since last tick * min(workers*0.75, 1)
                let tick = time.delta_seconds_f64() * (workers.0 as f64 * 0.75).min(1.0);
                difficulty_ticks.0 += tick;
            }
        } else if let Some(_player) = player {
            if workers.0 <= workers_cap.0 {
                let tick = time.delta_seconds_f64() * (workers_cap.0 - workers.0) as f64;
                difficulty_ticks.0 += tick;
            }
        }
    }
}

// Assign worker to game based on name
fn _assign_worker(mut query: Query<(&Name, &mut Workers)>, game_name: String) {
    for (name, mut workers) in query.iter_mut() {
        if name.0 == game_name {
            workers.0 += 1;
        }
    }
}

fn main() {
    App::new()
        .insert_resource(PrintTimer(Timer::from_seconds(3.0, TimerMode::Repeating)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(create_player)
        .add_startup_system(add_games)
        .add_system(difficulty_tick)
        .add_system(print_all)
        .run();
}
