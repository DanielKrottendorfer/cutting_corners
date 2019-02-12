


pub mod loaders{
    use assimp::Importer;
    use glium::index::PrimitiveType;

    use crate::my_game_engine::my_game_logic::my_renderer::{MyArmatureSkinVertex,VertexWeights,MyJoint,ModelRst,MyVertex,StaticMesh,AnimatedMesh};
    use cgmath::Matrix3;
    use cgmath::Quaternion;

    pub fn load_texture(display:&mut glium::Display, path:&str) -> glium::texture::SrgbTexture2d {
        let image = image::open(path).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        glium::texture::SrgbTexture2d::new(display, image).unwrap()
    }

    pub fn load_static_meshes(display:&mut glium::Display, path:&str) -> Vec<(ModelRst, StaticMesh)>{

        let mut smv :Vec<(ModelRst, StaticMesh)> = Vec::new();
        let mut importer = Importer::new();
        importer.triangulate(true);
        importer.generate_normals(|x| x.enable = true);

        let scene = importer.read_file(path).unwrap();

        for mesh in scene.mesh_iter() {

            let pos:Vec<[f32;3]>= mesh.vertex_iter().map(|v| v.into()).collect();
            let norm:Vec<[f32;3]>= mesh.normal_iter().map(|v| v.into()).collect();
            let tex:Vec<[f32;3]>= mesh.texture_coords_iter(0).map(|v| v.into()).collect();


            let mut verts:Vec<MyVertex> = Vec::new();

            for i in 0..pos.len(){
                verts.push(MyVertex {
                    position: pos[i],
                    normal: norm[i],
                    texture: [tex[i][0],tex[i][1]]
                });
            }

            let vb = glium::VertexBuffer::new(display, &verts);
            let mut indices:Vec<u16> = Vec::with_capacity(mesh.num_faces() as usize * 3);
            for face in mesh.face_iter() {
                indices.push(face[0] as u16);
                indices.push(face[1] as u16);
                indices.push(face[2] as u16);
            }

            let ib = glium::IndexBuffer::new(display, PrimitiveType::TrianglesList, &indices);

            let rotation;

            if path.ends_with(".dae"){
                rotation = Matrix4::from_angle_x(cgmath::Rad(std::f32::consts::PI*3.0/2.0));
            }else {
                rotation =  Matrix4::identity();
            }

            use cgmath::{Matrix4,SquareMatrix};
             let m =(
                ModelRst{
                    rotation: rotation,
                    scale: Matrix4::identity(),
                    translation: Matrix4::identity()
                }, StaticMesh{
                    vertices: (vb.unwrap()),
                    indices: (ib.unwrap())
            });
            smv.push(m);
        }
        smv
    }


    pub fn load_animated_collada_mesh(display:&mut glium::Display, path:&str) -> (ModelRst,AnimatedMesh){

        use cgmath::{Matrix4,SquareMatrix};
        let cd = collada::document::ColladaDocument::from_path(std::path::Path::new(path)).unwrap();
        let obj_set = cd.get_obj_set().unwrap();

        let mut triangles_v :Vec<((usize,usize,usize),(usize,usize,usize),(usize,usize,usize))> = Vec::new();

        let mut mesh : Vec<MyArmatureSkinVertex> = Vec::new();

        let mut vertex_weights :Vec<VertexWeights> = Vec::new();
        let mut indices :Vec<u16> = Vec::new();


        let vertices = &obj_set.objects[0].vertices;
        let normals = &obj_set.objects[0].normals;
        let textures = &obj_set.objects[0].tex_vertices;

        let bind_set = cd.get_bind_data_set().unwrap();

        let weights = &bind_set.bind_data[0].weights;

        for vw in &bind_set.bind_data[0].vertex_weights{
            if vw.vertex  as i16 > vertex_weights.len() as i16 -1 {
                let vn = VertexWeights{
                    joint_i: [vw.joint,0,0,0],
                    weights: [vw.weight,0,0,0],
                    joint_c: 1
                };
                vertex_weights.push(vn);
            }else {
                let vn = vertex_weights.last_mut().unwrap();
                vn.weights[vn.joint_c] = vw.weight;
                vn.joint_i[vn.joint_c] = vw.joint;
                vn.joint_c+=1;
            }
        }

        let obj = &obj_set.objects[0];

        for geo in &obj.geometry {
            for mesh in &geo.mesh {
                match mesh {
                    collada::PrimitiveElement::Triangles(triangles) => {
                        for triangle in &triangles.vertices {

                            let triangle_e = (((triangle.0).0,(triangle.0).2.unwrap(),(triangle.0).1.unwrap()),
                                              ((triangle.1).0,(triangle.1).2.unwrap(),(triangle.1).1.unwrap()),
                                              ((triangle.2).0,(triangle.2).2.unwrap(),(triangle.2).1.unwrap()));
                            triangles_v.push(triangle_e);
                        }
                    },
                    _ => ()
                }
            }
        }

        let mut i = 0;

        for triangle in &triangles_v {
            let wi = vertex_weights[(triangle.0).0].weights;
            let vertex1 = MyArmatureSkinVertex {
                position:   [vertices[(triangle.0).0].x as f32,vertices[(triangle.0).0].y as f32,vertices[(triangle.0).0].z as f32],
                normal:     [normals[(triangle.0).1].x as f32,normals[(triangle.0).1].y as f32,normals[(triangle.0).1].z as f32],
                texture:    [textures[(triangle.0).2].x as f32,textures[(triangle.0).2].y as f32],
                weights:    [weights[wi[0]],weights[wi[1]],weights[wi[2]],weights[wi[3]]],
                joint_mi:   vertex_weights[(triangle.0).0].joint_i,
                joint_c:    vertex_weights[(triangle.0).0].joint_c as u8
            };
            indices.push(i);
            i+=1;

            let wi = vertex_weights[(triangle.1).0].weights;
            let vertex2 = MyArmatureSkinVertex {
                position:   [vertices[(triangle.1).0].x as f32,vertices[(triangle.1).0].y as f32,vertices[(triangle.1).0].z as f32],
                normal:     [normals[(triangle.1).1].x as f32,normals[(triangle.1).1].y as f32,normals[(triangle.1).1].z as f32],
                texture:    [textures[(triangle.1).2].x as f32,textures[(triangle.1).2].y as f32],
                weights:    [weights[wi[0]],weights[wi[1]],weights[wi[2]],weights[wi[3]]],
                joint_mi:   vertex_weights[(triangle.1).0].joint_i,
                joint_c:    vertex_weights[(triangle.1).0].joint_c as u8
            };
            indices.push(i);
            i+=1;

            let wi = vertex_weights[(triangle.2).0].weights;
            let vertex3 = MyArmatureSkinVertex {
                position:   [vertices[(triangle.2).0].x as f32,vertices[(triangle.2).0].y as f32,vertices[(triangle.2).0].z as f32],
                normal:     [normals[(triangle.2).1].x as f32,normals[(triangle.2).1].y as f32,normals[(triangle.2).1].z as f32],
                texture:    [textures[(triangle.2).2].x as f32,textures[(triangle.2).2].y as f32],
                weights:    [weights[wi[0]],weights[wi[1]],weights[wi[2]],weights[wi[3]]],
                joint_mi:   vertex_weights[(triangle.2).0].joint_i,
                joint_c:    vertex_weights[(triangle.2).0].joint_c as u8
            };
            indices.push(i);
            i+=1;

            mesh.push(vertex1);
            mesh.push(vertex2);
            mesh.push(vertex3);
        }

        let animations = &cd.get_animations().unwrap();
        let joints = &cd.get_skeletons().unwrap()[0].joints;
        let inverse_bind_poses = &bind_set.bind_data[0].inverse_bind_poses.clone();

        let mut skeleton:Vec<MyJoint> = Vec::new();

        for i in 0.. animations.len() {

            let mut tsp_sample_poses:Vec<cgmath::Matrix4<f32>> = Vec::new();
            for y in animations[i].sample_poses.clone() {
                let mut nm:cgmath::Matrix4<f32> = y.into();
                nm.transpose_self();
                tsp_sample_poses.push(nm);
            }

            let mut tsp_inverse_bind_pose:cgmath::Matrix4<f32> = inverse_bind_poses[i].into();

            tsp_inverse_bind_pose.transpose_self();


            let translations:Vec<cgmath::Vector4<f32>> = Vec::new();
            let rotations:Vec<cgmath::Quaternion<f32>> = Vec::new();
            let lengths:Vec<f32> = Vec::new();

            skeleton.push(MyJoint{
                pose_m: tsp_sample_poses,
                translations: translations,
                rotations: rotations,
                lengths: lengths,
                time_stamps: animations[i].sample_times.clone(),
                inv_bind_pos: tsp_inverse_bind_pose,
                time_stamp_c: animations[i].sample_times.len() as u8,
                parent_i: joints[i].parent_index as i8
            });
        }
        let vb = glium::VertexBuffer::new(display, &mesh);
        let ib = glium::IndexBuffer::new(display, PrimitiveType::TrianglesList, &indices);
        let rotation = Matrix4::from_angle_x(cgmath::Rad(std::f32::consts::PI*3.0/2.0));

        let c: cgmath::Matrix4<f32> = cgmath::Matrix4::identity();
        let c:[[f32;4];4] = c.into();
        let current_transforms= [c;64];

        let mut m =(
            ModelRst{
                rotation: rotation,
                scale: Matrix4::identity(),
                translation: Matrix4::identity()
            }, AnimatedMesh{
                vertices: vb.unwrap(),
                indices: ib.unwrap(),
                skeleton: skeleton,
                current_pose: current_transforms,
                current_time_sec: 0.0,
                running: false
            }
        );

        let mut pose_transforms:Vec<Vec<Matrix4<f32>>> = Vec::new();
        for x in 0..m.1.skeleton[0].time_stamp_c as usize{
            let mut current_transforms:Vec<Matrix4<f32>> = Vec::new();
            for joint in &m.1.skeleton{
                let mut nm = joint.pose_m[x];
                let mut current_joint = joint;

                while current_joint.parent_i>=0{
                    current_joint = &m.1.skeleton[current_joint.parent_i as usize];
                    let pm = current_joint.pose_m[x];
                    nm = pm*nm;
                }
                current_transforms.push(nm);
            }
            pose_transforms.push(current_transforms);
        }


        for pose_t in &pose_transforms{
            let mut y = 0;
            for m1 in pose_t{

                let q = Matrix3{
                    x: m1.x.truncate(),
                    y: m1.y.truncate(),
                    z: m1.z.truncate()
                };
                let q:Quaternion<f32> = q.into();
                let t = m1.w;
                let l = (t.x.powf(2.0)+t.y.powf(2.0)+t.z.powf(2.0)).sqrt();
                m.1.skeleton[y].rotations.push(q);
                m.1.skeleton[y].translations.push(t);
                m.1.skeleton[y].lengths.push(l);
                y+=1;

            }
        }
        let mut i = 0;
        for pose in &pose_transforms{
            let mut y = 0;
            for mat in pose {
                println!("i:{} y:{} m:{:#?}",i,y,mat);
                y+=1;
            }
            i+=1;
        }

        m
    }

    pub fn load_static_collada_mesh(display:&mut glium::Display, path:&str) -> (ModelRst, StaticMesh){

        let cd = collada::document::ColladaDocument::from_path(std::path::Path::new(path)).unwrap();

        let obj_set = cd.get_obj_set().unwrap();

        let mut triangles_v :Vec<((usize,usize,usize),(usize,usize,usize),(usize,usize,usize))> = Vec::new();


        let mut mesh : Vec<MyVertex> = Vec::new();
        let mut indices : Vec<u16> = Vec::new();

        let mut i:u16 = 0;

        for obj in &obj_set.objects {

            let vertices = &obj.vertices;
            let normals = &obj.normals;
            let textures = &obj.tex_vertices;

            for geo in &obj.geometry {
                for mesh in &geo.mesh {
                    match mesh {
                        collada::PrimitiveElement::Triangles(triangles) => {
                            for triangle in &triangles.vertices {

                                let triangle_e = (((triangle.0).0,(triangle.0).2.unwrap(),(triangle.0).1.unwrap()),
                                                  ((triangle.1).0,(triangle.1).2.unwrap(),(triangle.1).1.unwrap()),
                                                  ((triangle.2).0,(triangle.2).2.unwrap(),(triangle.2).1.unwrap()));
                                triangles_v.push(triangle_e);
                            }
                        },
                        _ => ()
                    }
                }
            }

            for triangle in &triangles_v {
                let vertex1 = MyVertex {
                    position:   [vertices[(triangle.0).0].x as f32,vertices[(triangle.0).0].y as f32,vertices[(triangle.0).0].z as f32],
                    normal:     [normals[(triangle.0).1].x as f32,normals[(triangle.0).1].y as f32,normals[(triangle.0).1].z as f32],
                    texture:    [textures[(triangle.0).2].x as f32,textures[(triangle.0).2].y as f32],
                };
                indices.push(i);
                i+=1;
                let vertex2 = MyVertex {
                    position:   [vertices[(triangle.1).0].x as f32,vertices[(triangle.1).0].y as f32,vertices[(triangle.1).0].z as f32],
                    normal:     [normals[(triangle.1).1].x as f32,normals[(triangle.1).1].y as f32,normals[(triangle.1).1].z as f32],
                    texture:    [textures[(triangle.1).2].x as f32,textures[(triangle.1).2].y as f32],
                };
                indices.push(i);
                i+=1;
                let vertex3 = MyVertex {
                    position:   [vertices[(triangle.2).0].x as f32,vertices[(triangle.2).0].y as f32,vertices[(triangle.2).0].z as f32],
                    normal:     [normals[(triangle.2).1].x as f32,normals[(triangle.2).1].y as f32,normals[(triangle.2).1].z as f32],
                    texture:    [textures[(triangle.2).2].x as f32,textures[(triangle.2).2].y as f32],
                };
                indices.push(i);
                i+=1;

                mesh.push(vertex1);
                mesh.push(vertex2);
                mesh.push(vertex3);


            }
        }

        let vb = glium::VertexBuffer::new(display, &mesh);
        let ib = glium::IndexBuffer::new(display, PrimitiveType::TrianglesList, &indices);
        let rotation = Matrix4::from_angle_x(cgmath::Rad(std::f32::consts::PI*3.0/2.0));

        use cgmath::{Matrix4,SquareMatrix};
        let m =(
            ModelRst{
                rotation: rotation,
                scale: Matrix4::identity(),
                translation: Matrix4::identity()
            }, StaticMesh{
                vertices: (vb.unwrap()),
                indices: (ib.unwrap())
            });
        m
    }
    pub fn load_static_collada_mesh_rawdata(path:&str) -> (Vec<MyVertex>, Vec<u16>){
        let cd = collada::document::ColladaDocument::from_path(std::path::Path::new(path)).unwrap();

        let obj_set = cd.get_obj_set().unwrap();
        //let anm_set = cd.get_animations().unwrap();

        let mut triangles_v :Vec<((usize,usize,usize),(usize,usize,usize),(usize,usize,usize))> = Vec::new();


        let mut mesh : Vec<MyVertex> = Vec::new();
        let mut indices : Vec<u16> = Vec::new();

        let mut i:u16 = 0;

        for obj in &obj_set.objects {

            let vertices = &obj.vertices;
            let normals = &obj.normals;
            let textures = &obj.tex_vertices;

            for geo in &obj.geometry {
                for mesh in &geo.mesh {
                    match mesh {
                        collada::PrimitiveElement::Triangles(triangles) => {
                            for triangle in &triangles.vertices {

                                let triangle_e = (((triangle.0).0,(triangle.0).2.unwrap(),(triangle.0).1.unwrap()),
                                                  ((triangle.1).0,(triangle.1).2.unwrap(),(triangle.1).1.unwrap()),
                                                  ((triangle.2).0,(triangle.2).2.unwrap(),(triangle.2).1.unwrap()));
                                triangles_v.push(triangle_e);
                            }
                        },
                        _ => ()
                    }
                }
            }

            for triangle in &triangles_v {
                let vertex1 = MyVertex {
                    position:   [vertices[(triangle.0).0].x as f32,vertices[(triangle.0).0].y as f32,vertices[(triangle.0).0].z as f32],
                    normal:     [normals[(triangle.0).1].x as f32,normals[(triangle.0).1].y as f32,normals[(triangle.0).1].z as f32],
                    texture:    [textures[(triangle.0).2].x as f32,textures[(triangle.0).2].y as f32],
                };
                indices.push(i);
                i+=1;
                let vertex2 = MyVertex {
                    position:   [vertices[(triangle.1).0].x as f32,vertices[(triangle.1).0].y as f32,vertices[(triangle.1).0].z as f32],
                    normal:     [normals[(triangle.1).1].x as f32,normals[(triangle.1).1].y as f32,normals[(triangle.1).1].z as f32],
                    texture:    [textures[(triangle.1).2].x as f32,textures[(triangle.1).2].y as f32],
                };
                indices.push(i);
                i+=1;
                let vertex3 = MyVertex {
                    position:   [vertices[(triangle.2).0].x as f32,vertices[(triangle.2).0].y as f32,vertices[(triangle.2).0].z as f32],
                    normal:     [normals[(triangle.2).1].x as f32,normals[(triangle.2).1].y as f32,normals[(triangle.2).1].z as f32],
                    texture:    [textures[(triangle.2).2].x as f32,textures[(triangle.2).2].y as f32],
                };
                indices.push(i);
                i+=1;

                mesh.push(vertex1);
                mesh.push(vertex2);
                mesh.push(vertex3);


            }
        }
        (mesh,indices)
    }
}