
pub mod loaders{
    use crate::my_game_engine::my_game_logic::my_renderer;

    pub fn load_static_collada_mesh_rawdata(path:&str) -> (Vec<my_renderer::MyVertex>, Vec<u16>){
        let cd = collada::document::ColladaDocument::from_path(std::path::Path::new(path)).unwrap();

        let obj_set = cd.get_obj_set().unwrap();
        //let anm_set = cd.get_animations().unwrap();

        let mut triangles_v :Vec<((usize,usize,usize),(usize,usize,usize),(usize,usize,usize))> = Vec::new();


        let mut mesh : Vec<my_renderer::MyVertex> = Vec::new();
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
                let vertex1 = my_renderer::MyVertex {
                    position:   [vertices[(triangle.0).0].x as f32,vertices[(triangle.0).0].y as f32,vertices[(triangle.0).0].z as f32],
                    normal:     [normals[(triangle.0).1].x as f32,normals[(triangle.0).1].y as f32,normals[(triangle.0).1].z as f32],
                    texture:    [textures[(triangle.0).2].x as f32,textures[(triangle.0).2].y as f32],
                };
                indices.push(i);
                i+=1;
                let vertex2 = my_renderer::MyVertex {
                    position:   [vertices[(triangle.1).0].x as f32,vertices[(triangle.1).0].y as f32,vertices[(triangle.1).0].z as f32],
                    normal:     [normals[(triangle.1).1].x as f32,normals[(triangle.1).1].y as f32,normals[(triangle.1).1].z as f32],
                    texture:    [textures[(triangle.1).2].x as f32,textures[(triangle.1).2].y as f32],
                };
                indices.push(i);
                i+=1;
                let vertex3 = my_renderer::MyVertex {
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