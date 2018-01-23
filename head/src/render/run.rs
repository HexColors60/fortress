use sdl2;
use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseWheelDirection;

use render::setup::Setup;
use render::cursor::Cursor;
use render::render_game::RenderGame;
use render::gfx::GFX;

use game::model::Game;

use world::world_setup::WorldSetup;

use util::shapes::Point2;

pub fn run<'a>(setup: Setup, game: &'a Game) {
    let mut rgame = RenderGame::new(&setup, &game);
    let mut cursor = Cursor::new(
        setup.window_size.width as f32 / 2.0,
        setup.window_size.height as f32 / 2.0,
    );

    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();
    let mut window = video_subsys
        .window(
            setup.title,
            setup.window_size.width,
            setup.window_size.height,
        )
        .position_centered()
        .allow_highdpi()
        .resizable()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut events = sdl_context.event_pump().unwrap();

    render(&rgame, &cursor, &mut canvas);

    'main: loop {
        for event in events.wait_iter() {
            match event {
                Event::Quit { .. } | Event::AppTerminating { .. } => {
                    break 'main;
                }

                Event::Window {
                    win_event: WindowEvent::SizeChanged(width, height),
                    ..
                }
                | Event::Window {
                    win_event: WindowEvent::Resized(width, height),
                    ..
                } => {
                    rgame.on_resize(width as u32, height as u32);
                }

                //
                // User Input
                //
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    rgame.move_camera(-1, 0);
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    rgame.move_camera(1, 0);
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    rgame.move_camera(0, -1);
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    rgame.move_camera(0, 1);
                }
                Event::MouseButtonDown {
                    x, y, mouse_btn, ..
                } => {
                    // todo
                }

                Event::MouseMotion { x, y, .. } => {
                    cursor.xy(Point2::new(x as f32, y as f32));
                }

                Event::MouseWheel {
                    y,
                    direction: MouseWheelDirection::Normal,
                    ..
                } => {
                    rgame.on_mouse_scroll(y);
                }

                Event::MouseWheel {
                    y,
                    direction: MouseWheelDirection::Flipped,
                    ..
                } => {
                    rgame.on_mouse_scroll(-y);
                }

                _ => {}
            }

            // Re-render everything after each event.
            render(&rgame, &cursor, &mut canvas)
        }
    }

    println!("goodbye!");
}

fn render(rgame: &RenderGame, cursor: &Cursor, gfx: &mut GFX) {
    gfx.clear();

    rgame.render(gfx);

    gfx.finished_drawing();
}