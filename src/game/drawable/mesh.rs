#[derive(Copy, Clone)]
pub struct Vert {
    position: [f32; 3],
}

glium::implement_vertex!(Vert, position);

#[derive(Copy, Clone)]
pub struct Norm {
    normal: [f32; 3],
}

implement_vertex!(Norm, normal);

pub struct StaticMesh {
    pub vertices: glium::VertexBuffer<Vert>,
    pub normals: glium::VertexBuffer<Norm>,
    pub indices: glium::IndexBuffer<u16>,

    pub scale: cgmath::Matrix4<f32>,
    pub rotation: cgmath::Matrix4<f32>,
    pub translation: cgmath::Matrix4<f32>,
}

impl StaticMesh {
    pub fn load_obj(display: &mut glium::Display, path: &str) -> StaticMesh {
        use obj::*;
        use std::fs::File;
        use std::io::BufReader;

        println!("{}", path);
        let input = BufReader::new(File::open(path).unwrap());
        let demo: Obj = obj::load_obj(input).unwrap();
        let vertecis = demo.vertices;
        let indices: Vec<u16> = demo.indices;

        let mut positions: Vec<Vert> = Vec::new();
        let mut normals: Vec<Norm> = Vec::new();

        for i in vertecis {
            positions.push(Vert {
                position: i.position,
            });
            normals.push(Norm { normal: i.normal });
        }

        let vbp = glium::VertexBuffer::new(display, &positions).unwrap();
        let vbn = glium::VertexBuffer::new(display, &normals).unwrap();
        let ixb = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &indices,
        )
        .unwrap();

        let rot: cgmath::Matrix4<f32> = cgmath::Matrix4::from_angle_x(cgmath::Rad { 0: (0.0) });
        let trn: cgmath::Matrix4<f32> = cgmath::Matrix4::from_translation(cgmath::Vector3 {
            x: (0.0),
            y: (0.0),
            z: (2.0),
        });

        let scl: cgmath::Matrix4<f32> = cgmath::Matrix4::from_scale(0.01);

        StaticMesh {
            vertices: vbp,
            normals: vbn,
            indices: ixb,
            scale: scl,
            rotation: rot,
            translation: trn,
        }
    }
}
