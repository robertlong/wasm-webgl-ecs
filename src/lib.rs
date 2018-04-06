#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
extern crate specs;

use wasm_bindgen::prelude::*;

use specs::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! println {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct Engine {
    time: f32,
    last_time: f32,
    dt: f32,
    world: World,
    dispatcher: Dispatcher<'static, 'static>,
}

#[wasm_bindgen]
impl Engine {
    pub fn new() -> Engine {
        let mut world = World::new();

        world.register::<Pos>();
        world.register::<Vel>();

        // An entity may or may not contain some component.

        world.create_entity().with(Vel(2.0)).with(Pos(0.0)).build();
        world.create_entity().with(Vel(4.0)).with(Pos(1.6)).build();
        world.create_entity().with(Vel(1.5)).with(Pos(5.4)).build();

        // This entity does not have `Vel`, so it won't be dispatched.
        world.create_entity().with(Pos(2.0)).build();

        // This builds a dispatcher.
        // The third parameter of `with` specifies
        // logical dependencies on other systems.
        // Since we only have one, we don't depend on anything.
        // See the `full` example for dependencies.
        let dispatcher = DispatcherBuilder::new().with(SysA, "sys_a", &[]).build();

        Engine {
            time: 0.0f32,
            last_time: 0.0f32,
            dt: 0.0f32,
            world: world,
            dispatcher: dispatcher,
        }
    }

    pub fn play(&mut self, time: f32) {
        self.last_time = time;
    }

    pub fn update(&mut self, time: f32) {
        // Update time related state
        self.dt = time - self.time;
        self.last_time = self.time;
        self.time = time;

        self.dispatcher.dispatch(&mut self.world.res);

        println!("{}", self.dt);
    }

    pub fn pause(&mut self) {

    }
}

#[derive(Debug)]
struct Vel(f32);

impl Component for Vel {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Pos(f32);

impl Component for Pos {
    type Storage = VecStorage<Self>;
}

struct SysA;

impl<'a> System<'a> for SysA {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = (WriteStorage<'a, Pos>, ReadStorage<'a, Vel>);

    fn run(&mut self, (mut pos, vel): Self::SystemData) {
        // The `.join()` combines multiple components,
        // so we only access those entities which have
        // both of them.
        for (pos, vel) in (&mut pos, &vel).join() {
            pos.0 += vel.0;
            println!("{}", pos.0);
        }
    }
}

