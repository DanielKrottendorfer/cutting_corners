#[macro_use]
extern crate glium;

#[path = "./models/tuto-07-teapot.rs"]
mod teapot;

extern crate  cgmath;

use cgmath::Vector4;


#[path = "./game/dummy_game.rs"]
mod game;

fn main() {
    use glium::{glutin, Surface};

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new().with_title("123");
    let context = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices:glium::InderxBuffer = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                            &teapot::INDICES).unwrap();



    let vertex_shader_src = r#"
        #version 150

        in vec3 position;
        in vec3 normal;

        out vec3 v_normal;

        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 model;

        void main() {
            mat4 modelview = view * model;
            v_normal = transpose(inverse(mat3(modelview))) * normal;
            gl_Position = perspective * modelview * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 150

        in vec3 v_normal;
        out vec4 color;
        uniform vec3 u_light;

        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.6, 0.0, 0.0);
            vec3 regular_color = vec3(1.0, 0.0, 0.0);
            color = vec4(mix(dark_color, regular_color, brightness), 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src,
                                              None).unwrap();

    let mut closed = false;

    let mut pos = cgmath::Point3{
        x:2.0,
        y:-1.0,
        z:1.0
    };

    let dir = cgmath::Vector3{
        x:-2.0,
        y:1.0,
        z:1.0
    };

    let up_v = cgmath::Vector3{
        x:0.0,
        y:1.0,
        z:0.0
    };

    let fv = cgmath::Rad(3.141592 / 3.0);

    let mut left = false;
    let mut right = false;
    let mut forward = false;
    let mut backward = false;
    let mut up = false;
    let mut down = false;

    use std::time::{Duration,Instant};

    let time_per_loop = Duration::from_millis(1000/144);

    let game_time = Instant::now();

    let mut rotation:f32 = 0.0;

    //let mut prev_mpos = glutin::dpi::LogicalPosition::new(0.0, 0.0);

    while !closed {

        let start_time = game_time.elapsed();


        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        /*
        let mut model = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 2.0, 1.0f32]
        ];
        */
        let mut model:cgmath::Matrix4<f32> = cgmath::Matrix4{
            x: Vector4 {
                x: (0.01),
                y: (0.0),
                z: (0.0),
                w: (0.0)
            },
            y: Vector4 {
                x: (0.0),
                y: (0.01),
                z: (0.0),
                w: (0.0)
            },
            z: Vector4 {
                x: (0.0),
                y: (0.0),
                z: (0.01),
                w: (0.0)
            },
            w: Vector4 {
                x: (0.0),
                y: (0.0),
                z: (2.0),
                w: (1.0)
            }
        };

        rotation+=0.01;
        if rotation>=std::f32::consts::PI*2.0 {
            rotation=0.0;
        }
        let rot:cgmath::Matrix4<f32> = cgmath::Matrix4::from_angle_y(cgmath::Rad{ 0: (rotation) });

        model = model * rot;

        let view:cgmath::Matrix4<f32> = cgmath::Matrix4::look_at_dir(pos, dir, up_v);

        let perspective:cgmath::Matrix4<f32> = cgmath::perspective(fv,(4 / 3)as f32, 0.1, 1024.0);

        let light = [-1.0, 0.4, 0.9f32];

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        target.draw((&positions, &normals), &indices, &program,
                    &uniform! { model: cgmath::conv::array4x4(model), view: cgmath::conv::array4x4(view), perspective: cgmath::conv::array4x4(perspective), u_light: light },
                    &params).unwrap();
        target.finish().unwrap();

        if forward && !backward {
            pos+=dir*0.01;
        }else {
            if !forward && backward {
                pos-=dir*0.01;
            }
        }

        let rot_m = cgmath::Matrix4::from_angle_y(cgmath::Rad(std::f32::consts::PI/2.0));
        let mut dir_l = (rot_m *dir.extend(1.0)).truncate();
        dir_l.y = 0.0;
        if left && !right {
            pos+= dir_l *0.01;
        }else {
            if !left && right {
                pos-= dir_l *0.01;
            }
        }

        if up && !down {
            pos.y+=0.01;
        }else {
            if !up && down {
                pos.y-=0.01;
            }
        }

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed=true,
                    glutin::WindowEvent::KeyboardInput { input, .. } => {
                        match input.virtual_keycode {
                            Some(glutin::VirtualKeyCode::Escape) => closed=true,
                            Some(glutin::VirtualKeyCode::W) => if input.state == glutin::ElementState::Pressed { forward = true } else { forward = false },
                            Some(glutin::VirtualKeyCode::S) => if input.state == glutin::ElementState::Pressed { backward = true } else { backward = false },
                            Some(glutin::VirtualKeyCode::A) => if input.state == glutin::ElementState::Pressed { left = true } else { left = false },
                            Some(glutin::VirtualKeyCode::D) => if input.state == glutin::ElementState::Pressed { right = true } else { right = false },
                            Some(glutin::VirtualKeyCode::X) => if input.state == glutin::ElementState::Pressed { up = true } else { up = false },
                            Some(glutin::VirtualKeyCode::Y) => if input.state == glutin::ElementState::Pressed { down = true } else { down = false },
                            _ => (),
                        }
                    },
                    _ => (),
                },
                _ => ()
            }
        });

        let loop_time = game_time.elapsed() - start_time;

        if loop_time<time_per_loop{
            std::thread::sleep(time_per_loop-loop_time);
        }
    }
}
