
#![feature(globs)]

extern crate graphics;
extern crate piston;
extern crate sdl2_game_window;
extern crate opengl_graphics;

use std::rc::Rc;

use opengl_graphics::{
    Gl,
    Texture,
};
use sdl2_game_window::WindowSDL2;
use graphics::*;
use piston::{
    AssetStore,
    EventIterator,
    EventSettings,
    WindowSettings,
    Render,
    Update,
    Input,
};
use piston::sprite::*;
use piston::input;
use piston::image;
use piston::image::{
    GenericImage,
};
use piston::event::{
    Action,
    Sequence,
    Wait,
};

fn get_window_size(asset_store: &AssetStore) -> (u32, u32) {
    let path = asset_store.path("img/bg_0.png").unwrap();
    image::open(&path).unwrap().dimensions()
}

fn main() {
    let asset_store = AssetStore::from_folder("../bin/assets");
    let (width, height) = get_window_size(&asset_store);
    let opengl = piston::shader_version::opengl::OpenGL_3_2;
    let mut window = WindowSDL2::new(
        opengl,
        WindowSettings {
            title: "Swing Copters (Rust Clone)".to_string(),
            size: [width, height],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );

    let mut main_scene = Scene::new();

    // load resources
    let bg_tex = Rc::new(Texture::from_path(&asset_store.path("img/bg_0.png").unwrap()).unwrap());
    let mut bg = Sprite::from_texture(bg_tex.clone());
    bg.set_anchor(0.0, 0.0);
    main_scene.add_child(bg);

    let cloud_tex = Rc::new(Texture::from_path(&asset_store.path("img/cloud_0.png").unwrap()).unwrap());
    let mut cloud = Sprite::from_texture(cloud_tex.clone());
    cloud.set_anchor(0.5, 0.0);
    cloud.set_position(width as f64 / 2.0, 0.0);
    main_scene.add_child(cloud);

    let land_tex = Rc::new(Texture::from_path(&asset_store.path("img/land.png").unwrap()).unwrap());
    let (_, land_tex_height) = land_tex.get_size();
    let mut land = Sprite::from_texture(land_tex.clone());
    land.set_anchor(0.0, 0.0);
    land.set_position(0.0, (height - land_tex_height) as f64);
    main_scene.add_child(land);

    let mut wheel_textures = Vec::new();
    for i in range(0i, 4) {
        let path = asset_store.path(format!("img/wheels_{}.png", i).as_slice()).unwrap();
        wheel_textures.push(Rc::new(Texture::from_path(&path).unwrap()));
    }
    let mut wheel_texture_index = 0.0;
    let ref wheel_tex = wheel_textures[wheel_texture_index as uint];
    let mut wheel = Sprite::from_texture(wheel_tex.clone());
    wheel.set_anchor(0.5, 0.3);

    let copter_tex = Rc::new(Texture::from_path(&asset_store.path("img/bear_0_0_0.png").unwrap()).unwrap());
    let mut copter = Sprite::from_texture(copter_tex.clone());
    copter.set_anchor(0.5, 0.3);
    copter.set_position(width as f64 / 2.0, height as f64 / 2.0);
    copter.set_rotation(10.0);

    let wheel_id = copter.add_child(wheel);
    let copter_id = main_scene.add_child(copter);

    // Test actions
    main_scene.run_action(copter_id, &Sequence(vec![
        Action(Ease(EaseQuadraticOut, box MoveBy(2.0, 0.0, -200.0))),
        //Action(MoveBy(2.0, 0.0, -200.0)),
        Action(Ease(EaseQuinticIn, box ScaleTo(2.0, 2.0, 2.0))),
        Action(Blink(2.0, 5)),
        Action(FlipX(true)),
        Wait(2.0),
        Action(FlipX(false)),
        Action(Ease(EaseQuadraticIn, box FadeOut(2.0))),
        Action(Ease(EaseQuinticOut, box FadeIn(2.0))),
    ]));
    let a = Action(Ease(EaseQuadraticInOut, box RotateTo(4.0, 180.0)));
    main_scene.run_action(copter_id, &a);

    main_scene.run_action(wheel_id, &Sequence(vec![
        Action(Hide),
        Wait(2.0),
        Action(Show),
    ]));

    let event_settings = EventSettings {
        updates_per_second: 120,
        max_frames_per_second: 60,
    };
    let ref mut gl = Gl::new(opengl);
    for e in EventIterator::new(&mut window, &event_settings) {
        main_scene.update(&e);
        match e {
            Render(args) => {
                gl.viewport(0, 0, args.width as i32, args.height as i32);

                let c = Context::abs(args.width as f64, args.height as f64);

                main_scene.draw(&c, gl);
            },
            Update(args) => {
                wheel_texture_index = wheel_texture_index + args.dt * 20.0;
                let ref tex = wheel_textures[wheel_texture_index as uint % wheel_textures.len()];
                match main_scene.child_mut(wheel_id) {
                    Some(s) => s.set_texture(tex.clone()),
                    _ => {},
                }
            },
            Input(input::Press(_)) => {
                //main_scene.toggle_action(copter_id, &a);
                main_scene.remove_child(copter_id);
                println!("{}", main_scene.running_actions());
            },
            _ => {},
        }
    }
}

