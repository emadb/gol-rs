use gol::{Cell, Game, Position, Size};
use rand::Rng;
use ruscii::app::Config;
use ruscii::app::{App, State};
use ruscii::drawing::Pencil;
use ruscii::gui::FPSCounter;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::Window;

fn main() {
    let mut fps_counter = FPSCounter::default();

    let mut app = App::config(Config::new().fps(5));

    const SIZE: Size = Size(50);
    let mut rng = rand::rng();

    let mut game = Game::new(SIZE);

    // Random position
    for _ in 0..500 {
        let p = Position::new(
            rng.random_range::<usize, _>(0..SIZE.0),
            rng.random_range::<usize, _>(0..SIZE.0),
        );
        game.set_live(p);
    }
    // Glider
    // game.set_live(1, 2);
    // game.set_live(2, 3);
    // game.set_live(3, 1);
    // game.set_live(3, 2);
    // game.set_live(3, 3);
    app.run(|app_state: &mut State, window: &mut Window| {
        let mut pencil = Pencil::new(window.canvas_mut());
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                _ => (),
            }
        }

        fps_counter.update();
        pencil.draw_text(&format!("FPS: {}", fps_counter.count()), Vec2::xy(1, 1));

        for x in 0..SIZE.0 {
            for y in 0..SIZE.0 {
                let cell = game.get(Position::new(x, y));
                if *cell == Cell::Live {
                    pencil.draw_filled_rect('\u{25A9}', Vec2::xy(x, y), Vec2::xy(1, 1));
                }
            }
        }
        game.tick();
    });
}
