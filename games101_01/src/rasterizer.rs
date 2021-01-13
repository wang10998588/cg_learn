use std::iter::Map;
type Vector3i = cgmath::Vector3<i32>;
type Vector3f = cgmath::Vector3<f32>;
type Vector2f = cgmath::Vector2<f32>;
type Vector4f = cgmath::Vector4<f32>;
type Matrix4f = cgmath::Matrix4<f32>;

pub struct Rasterizer{
    model:Matrix4f,
    view:Matrix4f,
    projection:Matrix4f,
    pos_buf:Map<i32,Vec<Vector3f>>,
    ind_buf:Map<i32,Vec<Vector3i>>,
    frame_buf:Vec<Vector3f>,
    depth_buf:Vec<f32>,
    width:i32,
    height:i32,
    next_id:i32,
}

impl Rasterizer{
    pub fn new(w:i32,h:i32)->Rasterizer{

    }


}