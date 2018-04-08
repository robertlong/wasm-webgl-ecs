#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
extern crate specs;

use wasm_bindgen::prelude::*;

use specs::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = canvasRenderer)]
    fn clearCanvas();

    #[wasm_bindgen(js_namespace = canvasRenderer)]
    fn setFillColor(r: u8, g: u8, b: u8, a: f32);

    #[wasm_bindgen(js_namespace = canvasRenderer)]
    fn fillRect(x: i32, y: i32, width: i32, height: i32);

    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! println {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct Engine {
    world: World,
    dispatcher: Dispatcher<'static, 'static>,
}

#[wasm_bindgen]
impl Engine {
    pub fn new() -> Engine {
        let mut world = World::new();

        world.register::<Pos2d>();
        world.register::<Rect>();
        world.register::<Color>();

        world.add_resource(Time { time: 0.0, dt: 0.0 });

        world.create_entity()
            .with(Pos2d { x: 100, y: 0 })
            .with(Rect { width: 100, height: 100 })
            .with(Color{ r: 0, g: 0, b: 0, a: 1.0 })
            .build();

        let dispatcher = DispatcherBuilder::new()
            .with(CanvasRenderer, "canvas_renderer", &[])
            .with(TimeLogger, "time_logger", &[])
            .build();

        Engine {
            world: world,
            dispatcher: dispatcher,
        }
    }

    pub fn play(&mut self, time: f32) {
        let mut time_res = self.world.write_resource::<Time>();
        time_res.dt = 0.0;
        time_res.time = time;
    }

    pub fn update(&mut self, time: f32) {
        {
            // Update time related state
            let mut time_res = self.world.write_resource::<Time>();
            time_res.dt = time - time_res.time;
            time_res.time = time;
        }

        self.dispatcher.dispatch(&mut self.world.res);
    }

    pub fn pause(&mut self) {

    }
}

#[derive(Debug)]
struct Time {
    dt: f32,
    time: f32,
}

#[derive(Debug)]
struct Pos2d {
    x: i32,
    y: i32
}

impl Component for Pos2d {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Rect {
    width: i32,
    height: i32
}

impl Component for Rect {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: f32,
}

impl Component for Color {
    type Storage = VecStorage<Self>;
}

struct CanvasRenderer;

impl<'a> System<'a> for CanvasRenderer {
    type SystemData = (ReadStorage<'a, Pos2d>, ReadStorage<'a, Rect>, ReadStorage<'a, Color>);

    fn run(&mut self, (pos, rect, color): Self::SystemData) {
        clearCanvas();

        for (pos, rect, color) in (&pos, &rect, &color).join() {
            setFillColor(color.r, color.g, color.b, color.a);
            fillRect(pos.x, pos.y, rect.width, rect.height);
        }
    }
}

struct TimeLogger;

impl<'a> System<'a> for TimeLogger {
    type SystemData = Fetch<'a, Time>;

    fn run(&mut self, time: Self::SystemData) {
        println!("time: {} dt: {}", time.time, time.dt);
    }
}

