use crate::triangle::Triangle;
use std::collections::HashMap;
use cgmath::Zero;

type Vector3i = cgmath::Vector3<i32>;
type Vector3f = cgmath::Vector3<f32>;
type Vector2f = cgmath::Vector2<f32>;
type Vector4f = cgmath::Vector4<f32>;
type Matrix4f = cgmath::Matrix4<f32>;

pub enum Buffers{
    Color = 1,
    Depth = 2
}
pub enum Primitive{
    Line,
    Triangle
}

pub struct pos_buf_id{
    pub pos_id:i32
}

pub struct ind_buf_id{
    pub ind_id:i32
}

pub struct Rasterizer{
    model:Matrix4f,
    view:Matrix4f,
    projection:Matrix4f,
    pos_buf:HashMap<i32,Vec<Vector3f>>,
    ind_buf:HashMap<i32,Vec<Vector3i>>,
    frame_buf:Vec<Vector3f>,
    depth_buf:Vec<f32>,
    width:i32,
    height:i32,
    next_id:i32,
}

impl Rasterizer{
    pub fn new(w:i32,h:i32)->Rasterizer{
        Rasterizer{
            model:Matrix4f::from([0f32;4]),
            view:Matrix4f::from([0f32;4]),
            projection:Matrix4f::from([0f32;4]),
            pos_buf:HashMap::new(),
            ind_buf:HashMap::new(),
            frame_buf:Vec::new(),
            depth_buf:Vec::new(),
            width:w,
            height:h,
            next_id:0
        }
    }

    pub fn load_positions(&mut self,positions:Vec<Vector3f>)->pos_buf_id{
        let id = self.get_next_id();
        self.pos_buf.insert(id,positions);
        pos_buf_id{pos_id:id}
    }

    pub fn load_indices(&mut self,indices:Vec<Vector3i>)->ind_buf_id{
        let id = self.get_next_id();
        self.ind_buf.insert(id,indices);
        ind_buf_id{ind_id:id}
    }

    pub fn frame_buffer(&self)->&Vec<Vector3f>{
        &self.frame_buf
    }
    pub fn set_model(&mut self,m:Matrix4f){self.model = m;}
    pub fn set_view(&mut self,v:Matrix4f){self.view = v;}
    pub fn set_projection(&mut self,p:Matrix4f){self.projection = p;}

    pub fn clear(&mut self,buff:Buffers){
        match buff {
            Buffers::Color=>{
                for i in 0..self.frame_buf.len() {
                    self.frame_buf[i].set_zero();
                }
            },
            Buffers::Depth=>{

            }
        }
    }

    pub fn set_pixel(&mut self,point:&Vector3i,color:Vector3f){
        if point.x < 0 || point.x >= self.width
            || point.y<0||point.y>=self.height{
            return;
        }
        let ind = (self.height-1-point.y)*self.width + point.x;
        self.frame_buf[ind as usize] = color;
    }

    fn draw_line(&self,begin:Vector3f,end:Vector3f){

    }

    fn rasterize_wireframe(&self,t:&Triangle){

    }

    fn get_index(&self,x:i32,y:i32)->i32{
        (self.height-y)*self.width+x
    }

    fn get_next_id(&mut self)->i32{
        let ret = self.next_id;
        self.next_id+=1;
        ret
    }
}