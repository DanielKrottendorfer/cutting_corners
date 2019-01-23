extern crate cgmath;
extern crate glium;
extern crate time;
use crate::my_game::cgmath::InnerSpace;

#[path = "./drawable/mesh.rs"]
mod drawables;

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
}

impl DummyGame {
    pub fn new(title: &str) -> DummyGame {
        use glium::glutin;

        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new().with_title(title);
        let context = glutin::ContextBuilder::new()
            .with_depth_buffer(24)
            .with_vsync(true);
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

        let dir: cgmath::Vector3<f32> = cgmath::Vector3 {
            x: -2.0,
            y: 1.0,
            z: 1.0,
        };

        let dir = dir.normalize();

        let fv = cgmath::Rad(3.141592 / 3.0);
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
        }
    }

    pub fn input(&mut self) {
        use glutin::ElementState::Pressed;

        let mut close = false;

        let mut w: Keystate = Keystate::Nothing;
        let mut s: Keystate = Keystate::Nothing;
        let mut a: Keystate = Keystate::Nothing;
        let mut d: Keystate = Keystate::Nothing;
        let mut x: Keystate = Keystate::Nothing;
        let mut y: Keystate = Keystate::Nothing;

        self.events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => close = true,
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
        while !self.closed {
            self.input();
            self.update(1);
            self.render();
        }
    }

    pub fn start(&mut self) {
        self.game_loop();
    }
}
