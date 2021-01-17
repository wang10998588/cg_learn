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

pub struct PosBufId {
    pub pos_id:i32
}

pub struct IndBufId {
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
            model:Matrix4f::zero(),
            view:Matrix4f::zero(),
            projection:Matrix4f::zero(),
            pos_buf:HashMap::new(),
            ind_buf:HashMap::new(),
            frame_buf:Vec::new(),
            depth_buf:Vec::new(),
            width:w,
            height:h,
            next_id:0
        }
    }

    pub fn load_positions(&mut self,positions:Vec<Vector3f>)-> PosBufId {
        let id = self.get_next_id();
        self.pos_buf.insert(id,positions);
        PosBufId {pos_id:id}
    }

    pub fn load_indices(&mut self,indices:Vec<Vector3i>)-> IndBufId {
        let id = self.get_next_id();
        self.ind_buf.insert(id,indices);
        IndBufId {ind_id:id}
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
                for i in 0..self.depth_buf.len() {
                    self.depth_buf[i] = 0f32;
                }
            }
        }
    }

    pub fn draw(&mut self, pos_buffer: PosBufId, ind_buffer: IndBufId, _type:Primitive){

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
        let Vector3f{x:x1,y:y1, .. } = begin;
        let Vector3f { x:x2,y:y2,..} = end;
        let line_color = Vector3f::new(255.0,255.0,255.0);

        let dx = x2-x1;
        let dy = y2-y1;
        let dx1=fabsf32(dx);
        let dy1=fabsf32(dy);
        let px = 2.0*dy1-dx1;
        let py = 2.0*dx1-dy1;


    }

    fn rasterize_wireframe(&self,t:&Triangle){
        self.draw_line(t.c(),t.a());
        self.draw_line(t.c(),t.b());
        self.draw_line(t.b(),t.a());
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

fn to_vec4(v3:&Vector3f,w:f32)->Vector4f{
    Vector4f::new(v3.x,v3.y,v3.z,w)
}

fn fabsf32(f:f32)->f32{
    if f<0.0{
        return -f;
    }
    f
}