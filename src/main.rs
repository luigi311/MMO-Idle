use bevy::app::App;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
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
struct Worth(f64);

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
            "worth": 0.0001,
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
            Worth(game_data["worth"].as_f64().unwrap()),
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

fn print_player(
    time: Res<Time>,
    mut timer: ResMut<PrintTimer>,
    query: Query<(
        &Currency,
        &Workers,
        &WorkersCap,
        &DifficultyBase,
        &DifficultyGrowth,
        &DifficultyTicks,
        &RebornPoints,
        &IdleTokens,
        With<Player>,
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
            reborn_points,
            idle_tokens,
            _
        ) in query.iter()
        {
            println!("Player");
            println!("\tCurrency: \t{}", currency.0);
            println!("\tWorkers: \t{}", workers.0);
            println!("\tWorkers_cap: \t{}", workers_cap.0);
            println!("\tDiff_base: \t{}", difficulty_base.0);
            println!("\tDiff_grow: \t{}", difficulty_growth.0);
            println!("\tDiff_ticks: \t{}", difficulty_ticks.0);
            println!("\tReborn_points: \t{}", reborn_points.0);
            println!("\tIdle_tokens: \t{}", idle_tokens.0);
        }
    }
}

// Query all games and print them
fn print_games(
    time: Res<Time>,
    mut timer: ResMut<PrintTimer>,
    query: Query<(
        &Currency,
        &Workers,
        &WorkersCap,
        &DifficultyBase,
        &DifficultyGrowth,
        &DifficultyTicks,
        &Name,
        &Price,
        &Rewards,
        &Worth,
        &Subscription,
        &Population,
        With<Game>,
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
            name,
            price,
            rewards,
            worth,
            subscription,
            population,
            _
        ) in query.iter()
        {
            println!("{}", name.0);
            println!("\tPrice: \t\t{}", price.0);
            println!("\tSubscription: {}", subscription.0);
            println!("\tDiff_base: \t{}", difficulty_base.0);
            println!("\tDiff_grow: \t{}", difficulty_growth.0);
            println!("\tDiff_ticks: \t{}", difficulty_ticks.0);
            println!("\tRewards: \t{}", rewards.0);
            println!("\tWorth: \t\t{}", worth.0);
            println!("\tWorkers: \t{}", workers.0);
            println!("\tWorkers_cap: \t{}", workers_cap.0);
            println!("\tCurrency: \t{:.2}", currency.0);
            println!("\tPopulation: \t{}", population.0);
        }
    }
}

// Increase difficulty ticks
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

// Increase currency
fn currency_tick(
    time: Res<Time>,
    mut query: Query<(&mut Currency, &Workers, &Rewards, With<Game>)>,
) {
    for (mut currency, workers, rewards, _) in query.iter_mut() {
        if workers.0 > 0 {
            let tick = time.delta_seconds_f64() * (workers.0 as f64);        
            currency.0 += tick * rewards.0;
        }
    }
}

fn main() {
    App::new()
        .insert_resource(PrintTimer(Timer::from_seconds(3.0, TimerMode::Repeating)))
        .init_resource::<UiState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(create_player)
        .add_startup_system(add_games)
        .add_system(difficulty_tick)
        .add_system(currency_tick)
        .add_system(print_player)
        .add_system(print_games)
        .add_system(ui_system)
        .run();
}

fn number_format(number: f64) -> String {
    if number < 1000.0 {
        format!("{:.2}", number)
    } else if number < 1000000.0 {
        format!("{:.2}K", number / 1000.0)
    } else if number < 1000000000.0 {
        format!("{:.2}M", number / 1000000.0)
    } else if number < 1000000000000.0 {
        format!("{:.2}B", number / 1000000000.0)
    } else if number < 1000000000000000.0 {
        format!("{:.2}T", number / 1000000000000.0)
    } else {
        format!("{:.2}Q", number / 1000000000000000.0)
    }
}

#[derive(Default, Resource)]
struct UiState {
    workers_to_add: i64
}

fn ui_system(
    mut egui_context: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut query: Query<(
        &Name,
        &mut Workers,
        &WorkersCap,
        &Currency,
    )>
) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {        
        ui.horizontal(|ui| {
            // Show workers to add
            ui.label(format!("Workers to add: "));
            if ui.button("1").on_hover_text("Set workers to add to 1").clicked() {
                ui_state.workers_to_add = 1;
            };
            if ui.button("2X").on_hover_text("2x workers to add").clicked() {
                ui_state.workers_to_add *= 2;
            };
            if ui.button("X/2").on_hover_text("x/2 workers to add").clicked() {
                ui_state.workers_to_add /= 2;
            };
            if ui.button("-X").on_hover_text("Inverse count").clicked() {
                ui_state.workers_to_add *= -1;
            };
            

            ui.label(format!("{}", number_format(ui_state.workers_to_add as f64)));
        });

        // For each game,  add button to add/remove X workers, buttons to change x to 1, 2, X2 to add/remove, and show current workers
        for (name, mut workers, workers_cap, currency) in query.iter_mut() {
            ui.horizontal(|ui| {
                
                ui.label(&name.0);
                
                if ui.button("Add workers").on_hover_text("Add workers").clicked() {
                    if ui_state.workers_to_add > 0 {
                        workers.0 += ui_state.workers_to_add as u64;
                    } else {
                        if (ui_state.workers_to_add *-1) as u64 <= workers.0 {
                            workers.0 -= (ui_state.workers_to_add *-1) as u64;
                        } else {
                            workers.0 = 0;
                        }
                    }
                };

                if ui.button("Reset workers").on_hover_text("Reset workers").clicked() {
                    workers.0 = 0;
                };

                ui.horizontal(|ui| {
                    ui.label(format!("Workers: {}", number_format(workers.0 as f64)));
                    ui.label(format!("Workers cap: {}", number_format(workers_cap.0 as f64)));
                    ui.label(format!("Currency: {}", number_format(currency.0)));

                });
            });
        }
    });
}