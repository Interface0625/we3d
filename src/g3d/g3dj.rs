use serde::{Serialize, Deserialize};
use serde_json;

fn default_translation() -> [f32; 3] { [1.,1.,1.] }
fn default_rotation() -> [f32; 4] { [0.,0.,0.,1.] }
fn default_scale() -> [f32; 3] { [1.,1.,1.] }
fn default_node_children() -> Vec<Node> { Vec::new() }
fn default_node_parts() -> Vec<NodePart> { Vec::new() }
fn default_bone_translation() -> [f32; 4] { [1.,1.,1.,0.] }
fn default_bone_rotation() -> [f32; 4] { [0.,0.,0.,1.] }
fn default_bone_scale() -> [f32; 4] { [1.,1.,1.,0.] }



#[derive(Serialize, Deserialize)]
pub struct Mesh {
    pub attributes: Vec<String>,
    pub vertices: Vec<f32>,
    pub parts: Vec<MeshPart>,
}
#[derive(Serialize, Deserialize)]
pub struct MeshPart {
    pub id: String, 
    pub r#type: String,
    pub indices: Vec<u16>, 
}
#[derive(Serialize, Deserialize)]
pub struct Material {
    pub id: String, 
    pub ambient: [ f32; 3 ], 
    pub diffuse: [ f32; 3 ], 
    pub emissive: [ f32; 3 ], 
    pub opacity:  f32, 
    pub textures: Vec< Texture >,
}
#[derive(Serialize, Deserialize)]
pub struct Texture {
    pub id: String, 
    pub filename: String, 
    pub r#type: String
}
#[derive(Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    #[serde( default = "default_translation" )]
    pub translation: [f32; 3],
    #[serde( default = "default_rotation" )]
    pub rotation: [f32; 4],
    #[serde( default = "default_scale" )]
    pub scale: [f32; 3],
    #[serde( default = "default_node_children") ]
    pub children: Vec<Node>,
    #[serde( default = "default_node_parts") ]
    pub parts: Vec<NodePart>,
}
#[derive(Serialize, Deserialize)]
pub struct NodePart {
    pub meshpartid: String, 
    pub materialid: String, 
    pub bones: Vec<Bone>,
    pub uvMapping: [[  u16; 1 ];1]
}
#[derive(Serialize, Deserialize)]
pub struct Bone {
    node: String, 
    #[serde( default = "default_bone_translation" )]
    pub translation: [f32; 4],
    #[serde( default = "default_bone_rotation" )]
    pub rotation: [f32; 4],
    #[serde( default = "default_bone_scale" )]
    pub scale: [f32; 4],
}
#[derive(Serialize, Deserialize)]
pub struct Animation {
    pub id: String,
    pub bones: Vec<AnimationBone>,
}
#[derive(Serialize, Deserialize)]
pub struct AnimationBone {
    pub boneId: String,
    pub keyframes: Vec<KeyFrame>,
}
#[derive(Serialize, Deserialize)]
pub struct KeyFrame {
    pub keytime: f32,
    #[serde( default = "default_translation" )]
    pub translation: [f32; 3],
    #[serde( default = "default_rotation" )]
    pub rotation: [f32; 4],
    #[serde( default = "default_scale" )]
    pub scale: [f32; 3],
}

pub fn from_str(data: &String) -> G3dj { serde_json::from_str(data).unwrap() }

#[derive(Serialize, Deserialize)]
pub struct G3dj {
    version: [u8; 2],
    pub id: String,
    pub meshes: Vec< Mesh >,
    pub materials: Vec< Material >,
    pub nodes: Vec< Node >,
    pub animations: Vec< Animation >,
}
