
use cgmath::{Matrix4,InnerSpace,Rad,Vector2,Vector3};
use cgmath::Quaternion;

pub struct Cam {
    pub pos: cgmath::Point3<f32>,
    pub look_dir: cgmath::Vector3<f32>,
    pub ha: f32,
    pub va: f32,
    pub perspective: Matrix4<f32>,
    pub speed: f32
}

impl Cam{
    pub fn forward(&mut self, dt:&f32){
        self.pos += self.look_dir * self.speed * *dt;
    }
    pub fn backward(&mut self, dt:&f32){
        self.pos -= self.look_dir * self.speed * *dt;
    }

    pub fn move_angle(&mut self, dt:&f32, a:&f32){
        let ml:Matrix4<f32> = Matrix4::from_angle_y(Rad{ 0: (*a)});
        let mut ld = self.look_dir.extend(1.0);
        ld = ml * ld;
        if *a>std::f32::consts::PI*0.5 && *a<std::f32::consts::PI*1.5 {
            ld.y*=-1.0;
        }
        self.pos+=ld.truncate().normalize()* self.speed **dt;
    }
    pub fn left(&mut self, dt:&f32){
        let ml:Matrix4<f32> = Matrix4::from_angle_y(Rad{ 0: (std::f32::consts::PI*1.5)});
        let mut ld = self.look_dir.extend(1.0);
        ld = ml * ld;
        ld.y = 0.0;
        self.pos+=ld.truncate().normalize()* self.speed **dt;
    }
    pub fn right(&mut self, dt:&f32){
        let ml:Matrix4<f32> = Matrix4::from_angle_y(Rad{ 0: (std::f32::consts::PI*0.5)});
        let mut ld = self.look_dir.extend(1.0);
        ld = ml * ld;
        ld.y = 0.0;
        self.pos+=ld.truncate().normalize()* self.speed **dt;
    }
    pub fn up(&mut self, dt:&f32){
        self.pos += cgmath::Vector3{x:0.0,y:1.0,z:0.0} * self.speed * *dt;
    }
    pub fn down(&mut self, dt:&f32){
        self.pos -= cgmath::Vector3{x:0.0,y:1.0,z:0.0} * self.speed * *dt;
    }
    pub fn update_angle(&mut self){

        let xz = Vector2{x:self.look_dir.x,y:self.look_dir.z}.normalize();
        let yz = Vector2{x:self.look_dir.y,y:self.look_dir.z}.normalize();

        self.ha = yz.y.cos();
        self.va = xz.y.cos();

    }
    pub fn rotate(&mut self,ha:f32,va:f32){

        if self.va+va<std::f32::consts::PI/2.0{
            self.va = std::f32::consts::PI/2.0;
        }else  if self.va+va>(3.0*std::f32::consts::PI)/2.0 {
            self.va = (3.0*std::f32::consts::PI)/2.0 - 0.01;
        }else {self.va+=va;}

        self.ha+=ha;
        if self.ha >= std::f32::consts::PI*2.0 {
            self.ha-=std::f32::consts::PI*2.0
        }else if self.ha <= std::f32::consts::PI*2.0 {
            self.ha+=std::f32::consts::PI*2.0
        }

        self.look_dir = Vector3{
            x: (self.va.cos()*self.ha.sin()),
            y: (self.va.sin()),
            z: (self.va.cos()*self.ha.cos())
        }
    }
    pub fn look_at(&mut self,rst: &ModelRst){

        let t = rst.translation.w;
        let v = self.look_dir;
        self.look_dir = Vector3{
            x: (t.x - v.x),
            y: (t.y - v.y),
            z: (t.z - v.z)
        };

        //self.update_angle();
    }
}

#[derive(Copy, Clone)]
pub struct MyVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub texture: [f32; 2],
}
glium::implement_vertex!(MyVertex, position, normal, texture);

#[derive(Copy, Clone)]
pub struct MyArmatureSkinVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub texture: [f32; 2],
    pub weights: [f32; 4],
    pub joint_mi: [u8; 4],
    pub joint_c: u8
}
implement_vertex!(MyArmatureSkinVertex, position, normal, texture, weights, joint_mi, joint_c);

pub struct VertexWeights {
    pub joint_i: [u8;4],
    pub weights: [usize;4],
    pub joint_c: usize
}

pub struct MyJoint {
    pub pose_m: Vec<Matrix4<f32>>,
    pub translations: Vec<cgmath::Vector4<f32>>,
    pub rotations: Vec<cgmath::Quaternion<f32>>,
    pub lengths: Vec<f32>,
    pub time_stamps: Vec<f32>,
    pub inv_bind_pos: Matrix4<f32>,
    pub time_stamp_c: u8,
    pub parent_i: i8
}


pub struct ModelRst {
    pub rotation:Matrix4<f32>,
    pub scale:Matrix4<f32>,
    pub translation:Matrix4<f32>,
}

pub struct StaticMesh{
    pub vertices: glium::VertexBuffer<MyVertex>,
    pub indices: glium::IndexBuffer<u16>,
}

pub struct AnimatedMesh{
    pub vertices: glium::VertexBuffer<MyArmatureSkinVertex>,
    pub indices: glium::IndexBuffer<u16>,
    pub skeleton: Vec<MyJoint>,
    pub current_pose: [[[f32;4];4];64],
    pub current_time_sec: f32,
    pub running: bool
}

pub struct JointTransforms{
    transform_m: [[[f32;4];4]]
}
implement_buffer_content!(JointTransforms);
implement_uniform_block!(JointTransforms,transform_m);

impl AnimatedMesh {
    pub fn advance_time(&mut self,time_in_sec:&f32){
        let max_time = self.skeleton[0].time_stamps.last().unwrap();
        let current_time = &self.current_time_sec;
        if max_time<&(current_time+time_in_sec) {
            self.current_time_sec = current_time+time_in_sec-max_time
        }else { self.current_time_sec+=time_in_sec }
    }
    pub fn calculate_current_pose(&mut self) {
        use cgmath::SquareMatrix;
        
        let c: Matrix4<f32> = Matrix4::identity();
        let c:[[f32;4];4] = c.into();
        let mut current_transforms= [c;64];

        let skeleton = &self.skeleton;

        let mut y:usize = 0;

        for joint in skeleton {
            let mut prev_kf =0;
            let mut a= 0.0;
            let max_time = joint.time_stamps.last().unwrap();
            while self.current_time_sec>=*max_time {
                self.current_time_sec-=max_time;
            }

            for i in 0..joint.time_stamps.len() {
                if joint.time_stamps[i] < self.current_time_sec && self.current_time_sec < joint.time_stamps[i + 1] {
                    let t1 = joint.time_stamps[i];
                    let t2 = joint.time_stamps[i+1];
                    a= (self.current_time_sec-t1)/(t2-t1);
                    prev_kf = i;
                }
            }

            let q1 = joint.rotations[prev_kf];
            let q2 = joint.rotations[prev_kf+1];
            let q3 = q1.slerp(q2,a);

            let t1 = joint.translations[prev_kf].truncate();
            let t2 = joint.translations[prev_kf+1].truncate();
            let mut q5 = Quaternion::from_arc(t1,t2,None);
            q5.s=q5.s*a;
            let mut t3 = q5*t1;

            let len1 = joint.lengths[prev_kf];
            let len2 = joint.lengths[prev_kf+1];
            let len3 = len2*a + len1*(1.0-a);


            t3 = t3.normalize();

            t3*=len3;

            let mut mat:Matrix4<f32> = q3.into();
            mat.w = t3.extend(1.0);
            mat =   mat*joint.inv_bind_pos;
            mat.transpose_self();
            current_transforms[y] = mat.into();
            y=y+1;
        }
        self.current_pose = current_transforms ;
    }
}

pub struct Renderer {
    static_render_program: glium::Program,
    animated_render_program: glium::Program,
    static_textured_render_program: glium::Program,
}

impl Renderer {

    pub fn new(display: &mut glium::Display) -> Renderer{


        let vertex_shader = std::fs::read_to_string("./res/shader/vs.glsl").unwrap();
        let fragment_shader = std::fs::read_to_string("./res/shader/fs.glsl").unwrap();

        let vertex_shader_src: &str = vertex_shader.as_ref();
        let fragment_shader_src: &str = fragment_shader.as_ref();

        let program =
            glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
                .unwrap();

        let vertex_shadert = std::fs::read_to_string("./res/shader/dynvs.glsl").unwrap();
        let fragment_shadert = std::fs::read_to_string("./res/shader/fts.glsl").unwrap();

        let vertex_shadert_src: &str = vertex_shadert.as_ref();
        let fragment_shadert_src: &str = fragment_shadert.as_ref();

        let dprogram =
            glium::Program::from_source(display, vertex_shadert_src, fragment_shadert_src, None)
                .unwrap();

        let vertex_shadert = std::fs::read_to_string("./res/shader/vts.glsl").unwrap();
        let fragment_shadert = std::fs::read_to_string("./res/shader/fts.glsl").unwrap();

        let vertex_shadert_src: &str = vertex_shadert.as_ref();
        let fragment_shadert_src: &str = fragment_shadert.as_ref();

        let tprogram =
            glium::Program::from_source(display, vertex_shadert_src, fragment_shadert_src, None)
                .unwrap();

        Renderer{
            static_render_program: program,
            animated_render_program: dprogram,
            static_textured_render_program: tprogram,
        }

    }

    pub fn draw_static_mesh(&mut self,target_frame:&mut glium::Frame, cam :&Cam, model: &(ModelRst, StaticMesh)){

        use cgmath::{conv, Matrix4};
        use glium::Surface;
        target_frame.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
        let up_v = cgmath::Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let view: Matrix4<f32> = Matrix4::look_at_dir(cam.pos, cam.look_dir, up_v);

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

            let trs = &model.0;
            let trs_m = trs.translation * trs.rotation * trs.scale;

            let v_b = &model.1.vertices;
            let i_b = &model.1.indices;

            target_frame
                .draw(
                    v_b,
                    i_b,
                    &self.static_render_program,
                    &uniform! { model: conv::array4x4(trs_m), view: conv::array4x4(view),
                    perspective: conv::array4x4(cam.perspective) },
                    &params,
                )
                .unwrap();
    }



    pub fn draw_textured_static_mesh(&mut self,target_frame:&mut glium::Frame, cam :&Cam, model: &(ModelRst, StaticMesh), texture: &glium::texture::SrgbTexture2d){

        use cgmath::{conv, Matrix4};
        use glium::Surface;
        let up_v = cgmath::Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let view: Matrix4<f32> = Matrix4::look_at_dir(cam.pos, cam.look_dir, up_v);

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };


        let trs = &model.0;
        let trs_m = trs.translation * trs.rotation * trs.scale;

        let v_b = &model.1.vertices;
        let i_b = &model.1.indices;

        target_frame
            .draw(
                v_b,
                i_b,
                &self.static_textured_render_program,
                &uniform! { model: conv::array4x4(trs_m), view: conv::array4x4(view) ,myTextureSampler: texture,
                perspective: conv::array4x4(cam.perspective) },
                &params,
            )
            .unwrap();
    }

    pub fn draw_textured_animated_mesh(&mut self,target_frame:&mut glium::Frame,display:&glium::Display, cam :&Cam, model: &(ModelRst, AnimatedMesh), texture: &glium::texture::SrgbTexture2d){

        use cgmath::{conv, Matrix4};
        use glium::Surface;
        let up_v = cgmath::Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let view: Matrix4<f32> = Matrix4::look_at_dir(cam.pos, cam.look_dir, up_v);

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };


        let trs = &model.0;
        let trs_m = trs.translation * trs.rotation * trs.scale;

        let v_b = &model.1.vertices;
        let i_b = &model.1.indices;

        let mut buffer: glium::uniforms::UniformBuffer<JointTransforms> =
            glium::uniforms::UniformBuffer::empty_unsized(display, 4*4*4*64).unwrap();

        {
            let mut i = 0;
            let mut mapping = buffer.map();
            for val in mapping.transform_m.iter_mut() {
                *val = model.1.current_pose[i];
                i+=1;
            }
        }


        target_frame
            .draw(
                v_b,
                i_b,
                &self.animated_render_program,
                &uniform! { model: conv::array4x4(trs_m), view: conv::array4x4(view), perspective: conv::array4x4(cam.perspective), MyBlock: &buffer, myTextureSampler: texture,
                perspective: conv::array4x4(cam.perspective) },
                &params,
            )
            .unwrap();
    }
}