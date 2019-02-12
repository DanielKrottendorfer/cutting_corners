extern crate cgmath;
extern crate glium;
extern crate time;
use crate::my_game::cgmath::InnerSpace;

#[path = "./drawable/mesh.rs"]
mod drawables;

static MOUSE_SPEED: f32 = 0.005;

pub enum Keystate {
    Nothing,
    Pressed,
    Released,
}

pub struct Cam {
    pub pos: cgmath::Point3<f32>,
    pub look_dir: cgmath::Vector3<f32>,
    pub perspective: cgmath::Matrix4<f32>,
}


pub struct DummyGame {
    pub renderer: glium::Program,
    pub events_loop: glutin::EventsLoop,
    pub display: glium::Display,
    pub closed: bool,
    pub meshes: Vec<drawables::StaticMesh>,
    pub cam: Cam,
    pub keys_pressed: [bool; 6],
    pub previous_mouse_position: glutin::dpi::LogicalPosition,
}

impl DummyGame {
    pub fn new(title: &str, vsync: bool) -> DummyGame {
        use glium::glutin;

        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new().with_title(title);
        let context = glutin::ContextBuilder::new()
            .with_depth_buffer(24)
            .with_vsync(vsync);
        let mut display = glium::Display::new(window, context, &events_loop).unwrap();

        let vertex_shader = std::fs::read_to_string("./res/shader/vs.glsl").unwrap();
        let fragment_shader = std::fs::read_to_string("./res/shader/fs.glsl").unwrap();

        let vertex_shader_src: &str = vertex_shader.as_ref();
        let fragment_shader_src: &str = fragment_shader.as_ref();

        let program =
            glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
                .unwrap();

        let mut ms = Vec::new();

        ms.push(drawables::StaticMesh::load_obj(
            &mut display,
            "./res/teapot.obj",
        ));

        let pos = cgmath::Point3 {
            x: 2.0,
            y: -1.0,
            z: 1.0,
        };

        let look_dir: cgmath::Vector3<f32> = cgmath::Vector3 {
            x: -2.0,
            y: 1.0,
            z: 1.0,
        };

        let look_dir = look_dir.normalize();

        let fv = cgmath::Rad(std::f64::consts::PI / 3.0);
        let perspective: cgmath::Matrix4<f32> =
            cgmath::perspective(fv, (4 / 3) as f32, 0.1, 1024.0);

        //let kp = Vec::new();

        DummyGame {
            renderer: (program),
            events_loop: (events_loop),
            display: (display),
            closed: false,
            meshes: ms,
            cam: Cam {
                pos: (pos),
                look_dir: (dir),
                perspective: (perspective),
            },
            keys_pressed: [false, false, false, false, false, false],
            previous_mouse_position: glutin::dpi::LogicalPosition::new(0.0, 0.0),
        }
    }

    fn input(&mut self) {
        use glutin::ElementState::Pressed;

        let mut close = false;

        let mut w: Keystate = Keystate::Nothing;
        let mut s: Keystate = Keystate::Nothing;
        let mut a: Keystate = Keystate::Nothing;
        let mut d: Keystate = Keystate::Nothing;
        let mut x: Keystate = Keystate::Nothing;
        let mut y: Keystate = Keystate::Nothing;

        let mut ctrl_pressed = false;
        let mut size_changed = false;
        let fv = cgmath::Rad(std::f64::consts::PI / 3.0);
        let mut perspective_new: cgmath::Matrix4<f32> = 
            cgmath::perspective(fv, (4 / 3) as f32, 0.1, 1024.0);
        let mut current_position: glutin::dpi::LogicalPosition =
            glutin::dpi::LogicalPosition::new(0.0, 0.0);

        self.events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Resized(size) => {
                    size_changed = true;
                    let fv = cgmath::Rad(std::f64::consts::PI / 3.0);
                    perspective_new=cgmath::perspective(fv, (size.width / size.height)as f32, 0.1, 1024.0);
                    
                },
                glutin::WindowEvent::CloseRequested => close = true,
                glutin::WindowEvent::CursorMoved {
                    position,
                    modifiers,
                    ..
                } => {
                    if modifiers.ctrl {
                        ctrl_pressed = true;
                    }
                    current_position = position;
                }
                glutin::WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                    Some(glutin::VirtualKeyCode::Escape) => close = true,
                    Some(glutin::VirtualKeyCode::W) => {
                        if input.state == Pressed {
                            w = Keystate::Pressed
                        } else {
                            w = Keystate::Released
                        }
                    }
                    Some(glutin::VirtualKeyCode::S) => {
                        if input.state == Pressed {
                            s = Keystate::Pressed
                        } else {
                            s = Keystate::Released
                        }
                    }
                    Some(glutin::VirtualKeyCode::A) => {
                        if input.state == Pressed {
                            a = Keystate::Pressed
                        } else {
                            a = Keystate::Released
                        }
                    }
                    Some(glutin::VirtualKeyCode::D) => {
                        if input.state == Pressed {
                            d = Keystate::Pressed
                        } else {
                            d = Keystate::Released
                        }
                    }
                    Some(glutin::VirtualKeyCode::X) => {
                        if input.state == Pressed {
                            x = Keystate::Pressed
                        } else {
                            x = Keystate::Released
                        }
                    }
                    Some(glutin::VirtualKeyCode::Y) => {
                        if input.state == Pressed {
                            y = Keystate::Pressed
                        } else {
                            y = Keystate::Released
                        }
                    }
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        });

        self.closed = close;

        if size_changed {
            self.cam.perspective = perspective_new;
        }

        if ctrl_pressed {
            let dx: f64 = current_position.x - self.previous_mouse_position.x;
            let dy: f64 = current_position.y - self.previous_mouse_position.y;
            self.previous_mouse_position = current_position;
            if dx.abs() <= 5.0 || dy.abs() <= 5.0 {
                self.cam.h_angle += MOUSE_SPEED * -dx as f32;
                self.cam.v_angle += MOUSE_SPEED * -dy as f32;

                self.cam.adjust_angle();
            }
        }

        match w {
            Keystate::Pressed => self.keys_pressed[0] = true,
            Keystate::Released => self.keys_pressed[0] = false,
            _ => (),
        }
        match s {
            Keystate::Pressed => self.keys_pressed[1] = true,
            Keystate::Released => self.keys_pressed[1] = false,
            _ => (),
        }
        match a {
            Keystate::Pressed => self.keys_pressed[2] = true,
            Keystate::Released => self.keys_pressed[2] = false,
            _ => (),
        }
        match d {
            Keystate::Pressed => self.keys_pressed[3] = true,
            Keystate::Released => self.keys_pressed[3] = false,
            _ => (),
        }
        match x {
            Keystate::Pressed => self.keys_pressed[4] = true,
            Keystate::Released => self.keys_pressed[4] = false,
            _ => (),
        }
        match y {
            Keystate::Pressed => self.keys_pressed[5] = true,
            Keystate::Released => self.keys_pressed[5] = false,
            _ => (),
        }
    }

    fn update(&mut self, dt: i64) {
        use cgmath::Rad;

        let target_speed: f32 = 1.0;

        let mut dir: cgmath::Vector3<f32> = self.cam.look_dir;
        let mut speed = 0.0;
        let mut ang: Rad<f32>;
        ang = Rad(std::f32::consts::PI);

        if self.keys_pressed[0] && self.keys_pressed[2] {
            ang = Rad(2.0 * std::f32::consts::PI * (1.0 / 8.0));
            speed = target_speed;
        } else if self.keys_pressed[0] && self.keys_pressed[3] {
            ang = Rad(2.0 * std::f32::consts::PI * (7.0 / 8.0));
            speed = target_speed;
        } else if self.keys_pressed[1] && self.keys_pressed[2] {
            ang = Rad(2.0 * std::f32::consts::PI * (3.0 / 8.0));
            dir.y *= -1.0;
            speed = target_speed;
        } else if self.keys_pressed[1] && self.keys_pressed[3] {
            ang = Rad(2.0 * std::f32::consts::PI * (5.0 / 8.0));
            dir.y *= -1.0;
            speed = target_speed;
        } else if self.keys_pressed[0] && self.keys_pressed[1] {

        } else if self.keys_pressed[2] && self.keys_pressed[3] {

        } else if self.keys_pressed[2] {
            ang = Rad(2.0 * std::f32::consts::PI * (2.0 / 8.0));
            dir.y = 0.0;
            speed = target_speed;
        } else if self.keys_pressed[3] {
            ang = Rad(2.0 * std::f32::consts::PI * (3.0 / 4.0));
            dir.y = 0.0;
            speed = target_speed;
        } else if self.keys_pressed[1] {
            ang = Rad(std::f32::consts::PI);
            dir = dir.normalize();
            dir.y *= -1.0;
            speed = target_speed;
        } else if self.keys_pressed[0] {
            ang = Rad(0.0);
            dir = dir.normalize();
            speed = target_speed;
        }

        speed *= dt as f32;
        speed /= 1000.0;

        if self.keys_pressed[4] {
            self.cam.pos.y += target_speed * dt as f32 / 1000.0;
        }
        if self.keys_pressed[5] {
            self.cam.pos.y -= target_speed * dt as f32 / 1000.0;
        }

        let rot: cgmath::Matrix4<f32> = cgmath::Matrix4::from_angle_y(ang);

        //dir = dir.extend(1.0);
        let mut dir = dir.extend(1.0);
        dir = rot * dir;
        let mut dir = dir.truncate();

        dir *= speed;

        self.cam.pos += dir;
    }

    fn render(&self) {
        use cgmath::{conv, Matrix4};
        use glium::Surface;
        let mut target = self.display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
        let light = [-1.0, 0.4, 0.9f32];
        let up_v = cgmath::Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let view: Matrix4<f32> = Matrix4::look_at_dir(self.cam.pos, self.cam.look_dir, up_v);

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };

        for mesh in &self.meshes {
            let model = mesh.translation * mesh.rotation * mesh.scale;

            target
                .draw(
                    (&mesh.vertices, &mesh.normals),
                    &mesh.indices,
                    &self.renderer,
                    &uniform! { model: conv::array4x4(model), view: conv::array4x4(view),
                    perspective: conv::array4x4(self.cam.perspective), u_light: light },
                    &params,
                )
                .unwrap();
        }
        target.finish().unwrap();
    }

    fn game_loop(&mut self) {
        use time::PreciseTime;

        let mut previous = PreciseTime::now();
        let mut lag: i64 = 0;

        let mcs_per_update: i64 = 10000;
        
                let mut fpsc = 0;

                let mut start = PreciseTime::now();
        
        while !self.closed {
            self.input();

            let current = PreciseTime::now();
            let elapsed: i64;
            match previous.to(current).num_microseconds() {
                Some(x) => elapsed = x,
                None => elapsed = std::i64::MAX,
            }
            previous = current;
            lag += elapsed;

            while lag >= mcs_per_update {
                self.update(10);
                lag -= mcs_per_update;
            }
            
            let end = PreciseTime::now();
            fpsc+=1;
            if start.to(end).num_seconds()>=1{
                println!("fps: {}",fpsc);
                fpsc=0;
                start = PreciseTime::now();
            }
            
            self.render();
        }
    }

    pub fn start(&mut self) {
        self.game_loop();
    }
}
