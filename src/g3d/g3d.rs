use web_sys::WebGlRenderingContext as GL;
use web_sys::*;
use super::super::common_funcs as cf;
use super::super::shaders;
use nalgebra as na;
use na::{Vector2, Vector3, Vector4, Matrix4, Perspective3};
use super::g3dj;
use super::super::gl_helper as glh;
use super::super::camera;


pub struct Mesh {
    pub attributes: Vec<String>,
    pub vertices: Vec<f32>,
    buffer_vertices: WebGlBuffer,
    pub parts: Vec<MeshPart>,
}
impl Mesh {
    pub fn new(gl: &GL, mesh: &g3dj::Mesh)->Self{
        let parts = &mesh.parts;
        let _parts = parts.into_iter().map(|part| { MeshPart::new(gl, &part) } ).collect();
        let vertices = mesh.vertices.clone();
        Self {
            attributes: mesh.attributes.clone(),
            buffer_vertices: cf::init_vertex_buffer(gl, &vertices).unwrap(),
            vertices,
            parts: _parts,
        }
    }
}

pub struct MeshPart {
    pub id: String, 
    pub r#type: String,
    pub indices: Vec<u16>, 
    buffer_indices: WebGlBuffer,
}
impl MeshPart {
    pub fn new(gl: &GL, part: &g3dj::MeshPart)->Self{
        let indices = part.indices.clone();
        /*let mut indices = Vec::new();
        for i in 0..(part.indices.len() / 3) {
            indices.push(part.indices[i*3]);
            indices.push(part.indices[i*3+2]);
            indices.push(part.indices[i*3+1]);
        }*/
        Self {
            id: part.id.clone(),
            r#type: part.r#type.clone(),
            buffer_indices: cf::init_index_buffer(&gl, &indices).unwrap(),
            indices,
        }
    }
}

struct Material {
    id: String, 
    ambient: [ f32; 3 ], 
    diffuse: [ f32; 3 ], 
    emissive: [ f32; 3 ], 
    opacity:  f32, 
    textures: Vec< Texture >,
}

struct Texture {
    id: String, 
    filename: String, 
    r#type: String
}

struct Node {
    id: String,
    translation: [f32; 3],
    rotation: [f32; 4],
    scale: [f32; 3],
    children: Vec<Node>,
    parts: Vec<NodePart>,
}

struct NodePart {
    meshpartid: String, 
    materialid: String, 
    bones: Vec<Bone>,
    uvMapping: [[  u16; 1 ];1]
}

struct Bone {
    node: String, 
    translation: [f32; 4],
    rotation: [f32; 4],
    scale: [f32; 4],
}

struct Animation {
    id: String,
    bones: Vec<AnimationBone>,
}

struct AnimationBone {
    boneId: String,
    keyframes: Vec<KeyFrame>,
}

struct KeyFrame {
    keytime: f32,
    translation: [f32; 3],
    rotation: [f32; 4],
    scale: [f32; 3],
}

pub struct G3d {
    program: WebGlProgram,
    id: String,
    pub meshes: Vec< Mesh >,
    u_mvMatrix: WebGlUniformLocation,
    u_pMatrix: WebGlUniformLocation,
    
    //materials: Vec< Material >,
    //nodes: Vec< Node >,
    //animations: Vec< Animation >,
}

impl G3d {
    pub fn new(gl: &GL, json_str: &String) -> Self {
        let program = cf::link_program(&gl, shaders::vertex::skinned_g3d::SHADER, shaders::fragment::skinned_g3d::SHADER).unwrap();

        let u_mvMatrix = gl.get_uniform_location(&program, "u_mvMatrix").unwrap();
        let u_pMatrix = gl.get_uniform_location(&program, "u_pMatrix").unwrap();
        //let a_position = 
        let _g3dj = g3dj::from_str(json_str);
        let id = _g3dj.id;
        let meshes = _g3dj.meshes.into_iter().map(|mesh| { Mesh::new(gl, &mesh) } ).collect();
        Self {
            id,
            meshes,
            program,
            u_mvMatrix,
            u_pMatrix,
            //materials,
            //nodes,
            //animations,
        }
    }
    /* fn getKeyframe: function(time, boneAnimation){
        var keyframes = boneAnimation.keyframes,
            start, end, delta;
        /**
        var index = Math.floor(time*0.001);
        var time = time - (index*1000);
        var keyframes = boneAnimation.keyframes[index];
        var lastKeyframe = boneAnimation.keyframes[index+1][0];
        **/

        for(var i = 1; i < keyframes.length; i++){
            if(keyframes[i].keytime >= time){
                start = keyframes[i-1];
                end = keyframes[i];
                delta = (time - start.keytime) / (end.keytime - start.keytime);
                return start.lerp(end, delta);
            }
        }
        return keyframes[keyframes.length-1];
    }*/
    pub fn update(&self, time: f32){
        /*
        self.time_lerper.length = self.animations[self.currentAnimationIndex].length;
        let delta = self.time_lerper.update(now);

        let animation = me.animations[me.currentAnimationIndex];

        let frames = me.bones.map(function(boneAnimation){
            var frame = me.getKeyframe(time, boneAnimation);
            frame.boneId = boneAnimation.boneId;
            frame.node = boneAnimation.node;
            return frame;
        });
        for frame in frames {
            frame.node.localMatrix = frame.getMatrix();
        }

        function calcMatrices(node, parent) {
            node.modelMatrix = parent.model_matrix * node.local_matrix
            node.children.forEach( child => calcMatrices(child, node) );
        }
        calcMatrices(me.nodes.rootNode, {modelMatrix: new Matrix4()});

        let bones = self.bones;
        for bone in bones {
            for sub_bone in bone {
                sub_bone.model_matrix = sub_bone.node.model_matrix * sub_bone.inverseBindMatrix;
            }
        }

        let parts = self.parts;
        for part in parts {
            if part.bones.len() != 0 {
                part.boneArray = part.bones.reduce( a, bone => a.push(bone.model_matrix), [] )
            }
        }*/
    }
    fn bind_attribute_buffers (&self, gl: &GL, buffer: &WebGlBuffer){
        let size_of_gl_float = 4;

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(buffer));
        let attrib_ptr = gl.get_attrib_location(&self.program, "a_position") as u32;
        let stride = ( 3 + 3 + 2 + 8 ) * size_of_gl_float;
        gl.enable_vertex_attrib_array(attrib_ptr);
        gl.vertex_attrib_pointer_with_i32(attrib_ptr, 3, GL::FLOAT, false, stride, 0);

        let attrib_ptr = gl.get_attrib_location(&self.program, "a_normal") as u32;
        gl.enable_vertex_attrib_array(attrib_ptr);
        gl.vertex_attrib_pointer_with_i32(attrib_ptr, 3, GL::FLOAT, false, stride, 3 * size_of_gl_float);

        let attrib_ptr = gl.get_attrib_location(&self.program, "a_texturecoord") as u32;
        gl.enable_vertex_attrib_array(attrib_ptr);
        gl.vertex_attrib_pointer_with_i32(attrib_ptr, 2, GL::FLOAT, false, stride, (3+3) * size_of_gl_float);
    }

    pub fn render(&self, gl:&GL, camera: &camera::Camera){
        gl.use_program(Some(&self.program));
        glh::uniform_matrix4(gl, &self.u_mvMatrix, camera.v_matrix );
        glh::uniform_matrix4(gl, &self.u_pMatrix, camera.p_matrix );

        let meshes = &self.meshes;
        for mesh in meshes {
            self.bind_attribute_buffers(gl, &mesh.buffer_vertices);
            let parts = &mesh.parts;
            for part in parts {
                gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&part.buffer_indices));
                gl.draw_elements_with_i32(GL::TRIANGLES, part.indices.len() as i32, GL::UNSIGNED_SHORT, 0);
            }
        }
    }
}