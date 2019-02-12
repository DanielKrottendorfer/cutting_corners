
#[path = "./engine/renderer.rs"]
pub mod my_renderer;

#[path = "./engine/loaders.rs"]
pub mod my_loaders;

use std::thread;
use std::sync::mpsc;
use cgmath::Matrix4;
use std::sync::Arc;
use  std::sync::atomic::{AtomicUsize,Ordering};
use crate::my_game_engine::my_game_logic::my_renderer::{Renderer,Cam,ModelRst,MyVertex,StaticMesh,AnimatedMesh};

pub struct CCGame{
    pub models:Vec<(ModelRst, StaticMesh)>,
    pub animated_models:Vec<(ModelRst, AnimatedMesh)>,
    pub textures:Vec<glium::texture::SrgbTexture2d>,
    pub key_states:[glutin::ElementState;7],
    pub toggle_key_states:[glutin::ElementState;1],
    pub prev_cursor_pos:glutin::dpi::LogicalPosition,
    pub running: bool,
    pub resized : bool,
    pub mode_changed : bool,
    pub ego_mode : bool,
    pub window_size: glutin::dpi::LogicalSize,
    pub moved : bool,
    pub window_position: glutin::dpi::LogicalPosition,
    pub cam:Cam,
    pub rx:mpsc::Receiver<(Vec<MyVertex>, Vec<u16>)>,
    pub tx:mpsc::Sender<(Vec<MyVertex>, Vec<u16>)>,
    pub ready_in_que: Arc<AtomicUsize>,
    pub mics_alive: f64
}

impl CCGame{

    pub fn new() -> CCGame{
        let rst_v: Vec<(ModelRst, StaticMesh)> = Vec::new();
        let rst_av: Vec<(ModelRst, AnimatedMesh)> = Vec::new();
        let pos = cgmath::Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        let look_dir: cgmath::Vector3<f32> = cgmath::Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        use cgmath::InnerSpace;
        let look_dir = look_dir.normalize();

        let fv = cgmath::Rad(std::f32::consts::PI / 3.0);
        let perspective: cgmath::Matrix4<f32> = cgmath::perspective(fv, (4 / 3) as f32, 0.1, 1024.0);

        let vt = Vec::new();

        let (tx, rx) = mpsc::channel();

        CCGame{
            models: rst_v,
            animated_models: rst_av,
            textures: vt,
            key_states: [glutin::ElementState::Released;7],
            toggle_key_states: [glutin::ElementState::Released;1],
            prev_cursor_pos: glutin::dpi::LogicalPosition {x:0.0,y:0.0},
            running : true,
            resized : false,
            mode_changed: false,
            ego_mode: true,
            window_size: glutin::dpi::LogicalSize {height:400.0,width:300.0},
            moved : false,
            window_position: glutin::dpi::LogicalPosition {x:0.0,y:0.0},
            cam: Cam {
                pos,
                look_dir,
                ha: std::f32::consts::PI,
                va: std::f32::consts::PI,
                perspective,
                speed: 0.001
            },
            rx: (rx),
            tx: (tx),
            ready_in_que: Arc::new(AtomicUsize::new(0)),
            mics_alive: 0.0
        }
    }

    pub fn init(&mut self,display:&mut glium::Display){

        /*

        let mut m = my_loaders::loaders::load_static_collada_mesh(display,"./res/cubeStackBendingRotating.dae");

        let t:cgmath::Matrix4<f32> = cgmath::Matrix4::from_translation(cgmath::Vector3{
            x: (1.0),
            y: (0.0),
            z: (1.0)
        });

        let s:cgmath::Matrix4<f32> = cgmath::Matrix4::from_scale(0.1);

        m.0.translation = t;
        m.0.scale = s;

        self.models.push(m);

        */
        let mut am = my_loaders::loaders::load_animated_collada_mesh(display,"./res/untitled.dae");

        let t:cgmath::Matrix4<f32> = cgmath::Matrix4::from_translation(cgmath::Vector3{
            x: (0.0),
            y: (0.0),
            z: (1.0)
        });

        let s:cgmath::Matrix4<f32> = cgmath::Matrix4::from_scale(0.1);

        am.0.translation = t;
        am.0.scale = s;
        am.1.running = true;

        self.animated_models.push(am);
        let mut am = my_loaders::loaders::load_animated_collada_mesh(display,"./res/untitled.dae");

        let t:cgmath::Matrix4<f32> = cgmath::Matrix4::from_translation(cgmath::Vector3{
            x: (1.0),
            y: (0.0),
            z: (1.0)
        });

        let s:cgmath::Matrix4<f32> = cgmath::Matrix4::from_scale(0.1);

        am.0.translation = t;
        am.0.scale = s;
        am.1.running = true;

        self.animated_models.push(am);
        let mut am = my_loaders::loaders::load_animated_collada_mesh(display,"./res/untitled.dae");

        let t:cgmath::Matrix4<f32> = cgmath::Matrix4::from_translation(cgmath::Vector3{
            x: (2.0),
            y: (0.0),
            z: (1.0)
        });

        let s:cgmath::Matrix4<f32> = cgmath::Matrix4::from_scale(0.1);

        am.0.translation = t;
        am.0.scale = s;
        am.1.running = true;

        self.animated_models.push(am);
        let mut am = my_loaders::loaders::load_animated_collada_mesh(display,"./res/untitled.dae");

        let t:cgmath::Matrix4<f32> = cgmath::Matrix4::from_translation(cgmath::Vector3{
            x: (3.0),
            y: (0.0),
            z: (1.0)
        });

        let s:cgmath::Matrix4<f32> = cgmath::Matrix4::from_scale(0.1);

        am.0.translation = t;
        am.0.scale = s;
        am.1.running = true;

        self.animated_models.push(am);


        let texture = my_loaders::loaders::load_texture(display, "./res/cubeTex.png");
        self.textures.push(texture);
    }

    pub fn input(&mut self,events_loop :&mut glutin::EventsLoop){

        use glutin::ElementState::{Pressed,Released};
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::DeviceEvent{ event, ..} => match event {
                    glutin::DeviceEvent::MouseMotion { delta, ..} => {
                        if self.ego_mode{
                            let mouse_speed:f32 = 0.001;
                            self.cam.rotate(-delta.0 as f32*mouse_speed,delta.1 as f32*mouse_speed);
                        }
                    },
                    _ => ()
                },
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Resized(size) => {
                        let fv = cgmath::Rad(std::f32::consts::PI / 3.0);
                        let perspective: cgmath::Matrix4<f32> = cgmath::perspective(fv, (size.width / size.height) as f32, 0.1, 1024.0);
                        self.cam.perspective = perspective;
                        self.window_size = size;
                    },
                    glutin::WindowEvent::CloseRequested => self.running = false,
                    glutin::WindowEvent::KeyboardInput { input, .. } => {
                        match input.virtual_keycode {
                            Some(glutin::VirtualKeyCode::W) => if input.state == Pressed {self.key_states[0] = Pressed} else {self.key_states[0] = Released} ,
                            Some(glutin::VirtualKeyCode::S) => if input.state == Pressed {self.key_states[1] = Pressed} else {self.key_states[1] = Released} ,
                            Some(glutin::VirtualKeyCode::A) => if input.state == Pressed {self.key_states[2] = Pressed} else {self.key_states[2] = Released} ,
                            Some(glutin::VirtualKeyCode::D) => if input.state == Pressed {self.key_states[3] = Pressed} else {self.key_states[3] = Released} ,
                            Some(glutin::VirtualKeyCode::X) => if input.state == Pressed {self.key_states[4] = Pressed} else {self.key_states[4] = Released} ,
                            Some(glutin::VirtualKeyCode::Y) => if input.state == Pressed {self.key_states[5] = Pressed} else {self.key_states[5] = Released} ,
                            Some(glutin::VirtualKeyCode::LShift) => if input.state == Pressed {self.key_states[6] = Pressed} else {self.key_states[6] = Released} ,
                            Some(glutin::VirtualKeyCode::F) => if input.state == Pressed {self.mode_changed = true; if self.toggle_key_states[0] == Pressed {self.toggle_key_states[0] = Released} else { self.toggle_key_states[0] = Pressed }} ,
                            Some(glutin::VirtualKeyCode::Escape) => self.running = false,
                            Some(glutin::VirtualKeyCode::Space) => if input.state == Pressed{
                                /*if self.animated_models[0].1.running {self.animated_models[0].1.running = false} else { self.animated_models[0].1.running = true }*/},
                            _ => (),
                        }
                    },
                    _ => (),
                },
                _ => ()
            }
        });
    }

    pub fn load_que(&mut self, display: &mut glium::Display) {
        match self.rx.try_recv() {
            Ok(x) => {
                let vb = glium::VertexBuffer::new(display, &x.0).unwrap();
                let ib = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &x.1).unwrap();
                let scale = Matrix4::from_scale(0.1);
                let translation = Matrix4::from_translation(cgmath::Vector3 {
                    x: (self.models.len() as f32),
                    y: (0.0),
                    z: (2.0)
                });
                let rotation: Matrix4<f32> = cgmath::SquareMatrix::identity();

                self.models.push((ModelRst {
                        rotation: rotation,
                        scale: scale,
                        translation: translation
                },  StaticMesh {
                        vertices: (vb),
                        indices: (ib)
                }));
            },
            Err(e) => println!("{}", e)
        }
    }

    pub fn update(&mut self,dt: &f32){


        for animated_model in &mut self.animated_models{
            if animated_model.1.running{
                animated_model.1.advance_time(&(dt/1000.0));
            }
        }

        use glutin::ElementState::Pressed;

        if (self.key_states[0] == Pressed && self.key_states[1] == Pressed) || (self.key_states[2] == Pressed && self.key_states[3] == Pressed ) || (self.key_states[4] == Pressed && self.key_states[5] == Pressed ){
            return;
        }

        if self.key_states[0] == Pressed && self.key_states[2] == Pressed {
            self.cam.move_angle(dt, &(std::f32::consts::PI*0.25));
        }else if self.key_states[0] == Pressed && self.key_states[3] == Pressed {
            self.cam.move_angle(dt, &(std::f32::consts::PI * 1.75));
        } else if self.key_states[1] == Pressed && self.key_states[2] == Pressed {
            self.cam.move_angle(dt, &(std::f32::consts::PI*0.75));
        }else if self.key_states[1] == Pressed && self.key_states[3] == Pressed {
            self.cam.move_angle(dt, &(std::f32::consts::PI * 1.25));
        }else if self.key_states[0] == Pressed {
            self.cam.forward(dt);
        }else if self.key_states[1] == Pressed {
            self.cam.backward(dt);
        }else if self.key_states[2] == Pressed {
            self.cam.right(dt);
        }else if self.key_states[3] == Pressed {
            self.cam.left(dt);
        }else if self.key_states[4] == Pressed {
            self.cam.up(dt);
        }else if self.key_states[5] == Pressed {
            self.cam.down(dt);
        }

        if self.key_states[6] == Pressed {
            self.cam.speed = 0.005;
        }else {
            self.cam.speed = 0.001;
        }

        if self.toggle_key_states[0] == Pressed {
            self.ego_mode = false;
        }else {
            self.ego_mode= true
        }
    }

    pub fn render(&mut self,renderer: &mut Renderer,target_frame: &mut glium::Frame,display:&mut glium::Display){

        for model in &mut self.animated_models{
            model.1.calculate_current_pose();
        }

        for model in &self.models{
            renderer.draw_textured_static_mesh(target_frame,&self.cam,model,&self.textures[0]);
        }
        for model in &self.animated_models{
            renderer.draw_textured_animated_mesh(target_frame,display,&self.cam,model,&self.textures[0]);
        }
    }
}
