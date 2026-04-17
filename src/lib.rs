use quartz::*;
use ramp::prism;

mod constants;

use constants::*;

fn build_game_scene(_ctx: &mut prism::Context) -> Scene {
    const BG_PAD: f32 = 60.0;
    let bg_w = VW + BG_PAD * 2.0;
    let bg_h = VH + BG_PAD * 2.0;
    let bg = GameObject::build("bg")
        .size(bg_w, bg_h)
        .position(0.0, 0.0)
        .layer(0)
        .ignore_zoom()
        .finish();

    let mut scene = Scene::new("game").with_object("bg", bg);

    scene
}

pub struct App;

impl App {
    #![allow(clippy::new_ret_no_self)]
    fn new(ctx: &mut Context, _assets: Assets) -> impl Drawable {
        let mut canvas = Canvas::new(ctx, CanvasMode::Landscape);
        // canvas.add_scene(build_game_scene(ctx));
        // canvas.load_scene("game");
        canvas
    }
}

ramp::run! { |ctx: &mut Context, assets: Assets| { App::new(ctx, assets) } }
