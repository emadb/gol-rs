use gol::{Cell, Game, Location, Size};
use rand::Rng;
use ruscii::app::Config;
use ruscii::app::{App, State};
use ruscii::drawing::Pencil;
use ruscii::gui::FPSCounter;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::Window;

const SIZE: Size = Size(50);
const INITIAL_LIVE_CELLS: usize = 500;
const FPS: u32 = 5;

fn initialize_game(rng: &mut impl Rng) -> Game {
    let mut game = Game::new(SIZE);

    for _ in 0..INITIAL_LIVE_CELLS {
        let p = Location::new(rng.random_range(0..SIZE.0), rng.random_range(0..SIZE.0));
        game.set_live(p);
    }

    // Uncomment to add a glider
    // game.set_live(Position::new(1, 2));
    // game.set_live(Position::new(2, 3));
    // game.set_live(Position::new(3, 1));
    // game.set_live(Position::new(3, 2));
    // game.set_live(Position::new(3, 3));

    game
}

fn handle_keyboard_events(app_state: &mut State) {
    for key_event in app_state.keyboard().last_key_events() {
        if let KeyEvent::Pressed(Key::Esc) | KeyEvent::Pressed(Key::Q) = key_event {
            app_state.stop();
        }
    }
}

fn draw_game(pencil: &mut Pencil, game: &Game) {
    for x in 0..SIZE.0 {
        for y in 0..SIZE.0 {
            let cell = game.get(Location::new(x, y));
            if *cell == Cell::Live {
                pencil.draw_filled_rect('\u{25A9}', Vec2::xy(x, y), Vec2::xy(1, 1));
            }
        }
    }
}

fn main() {
    let mut fps_counter = FPSCounter::default();
    let mut app = App::config(Config::new().fps(FPS));
    let mut rng = rand::rng();
    let mut game = initialize_game(&mut rng);

    app.run(move |app_state: &mut State, window: &mut Window| {
        let mut pencil = Pencil::new(window.canvas_mut());

        handle_keyboard_events(app_state);

        fps_counter.update();
        pencil.draw_text(&format!("FPS: {}", fps_counter.count()), Vec2::xy(1, 1));

        draw_game(&mut pencil, &game);
        game.tick();
    });
}
