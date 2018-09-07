extern crate piston_window;
extern crate find_folder;
extern crate rand;

use piston_window::*;
use piston_window::math::Vec2d;
use rand::prelude::random;


const FONT_FAMILY :&str = "firaSans";
const TYPE_FACE :&str = "FiraSans-Bold.ttf";
const FONT_SIZE :piston_window::types::FontSize = 256;

const WIDTH :u32 = 1920;
const HEIGHT :u32 = 1080;

const DEFAULT_TEXT :&str = "BABY!";
const QUIT :&str = "quit";

fn main() {
    let quit_string = String::from(QUIT);

//    let fonts = find_folder::Search::ParentsThenKids(2, 3)
//        .for_folder(FONT_FAMILY).unwrap();
    //    println!("{:?}", fonts);
//    let ref font = fonts.join(TYPE_FACE);

    let font_bytes :&[u8] = include_bytes!("../fonts/firaSans/FiraSans-Bold.ttf");

    let mut window: PistonWindow =
        WindowSettings::new("Alphabangrs", [WIDTH, HEIGHT])
            .opengl(OpenGL::V3_3)
            .decorated(true)
            .fullscreen(true)
            .srgb(false)
            .samples(2)
            .exit_on_esc(false).build().unwrap();
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::from_bytes(&font_bytes, factory, TextureSettings::new()).unwrap();
//    let mut glyphs = Glyphs::new(font_bytes, factory, TextureSettings::new()).unwrap();


    window.set_max_fps(60);
    window.set_capture_cursor(true);

    let mut text = String::from("type 'quit' to exit");
    let mut color = [0.0, 0.0, 1.0, 1.0];
    let mut typed = String::new();
    while let Some(e) = window.next() {
        if let Some(text_val) = e.text_args() {
            let text_test = text_val.trim();
            if let  Some(ch) = text_test.chars().next() {
                if ch.is_alphanumeric() {
                    text = String::from(text_test);
                    typed.push_str(text.to_lowercase().as_ref());

                    if typed.eq(&quit_string) {
                        window.set_should_close(true);
                    } else if ! quit_string.starts_with(&typed) {
                        typed.clear();
                    }
                } else {
                    text = String::from(DEFAULT_TEXT);
                    typed.clear();
                }
                color = [random(), random(), random(), 1.0];
            }
        }

        window.draw_2d(&e, |c, g| {
            let view_size :Vec2d = c.get_view_size();
            let transform = c.transform.trans((view_size[0] as f64 / 2.0) - ((text.len() as f64 / 2.0) * (FONT_SIZE  as f64 / 2.0)) , view_size[1] as f64 / 2.0);
            clear([1.0, 1.0, 1.0, 1.0], g);
            text::Text::new_color(color, FONT_SIZE).draw(
                text.as_ref(),
                &mut glyphs,
                &c.draw_state,
                transform, g
            ).unwrap();
        });
    }
}
