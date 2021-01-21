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
    model:Option<Matrix4f>,
    view:Option<Matrix4f>,
    projection:Option<Matrix4f>,
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
            model:Option::None,
            view:Option::None,
            projection:Option::None,
            pos_buf:HashMap::new(),
            ind_buf:HashMap::new(),
            frame_buf:vec![Vector3f::zero(); (w * h) as usize],
            depth_buf:vec![0.0; (w * h) as usize],
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
    pub fn set_model(&mut self,m:Matrix4f){self.model = Some(m);}
    pub fn set_view(&mut self,v:Matrix4f){self.view = Some(v);}
    pub fn set_projection(&mut self,p:Matrix4f){self.projection = Some(p);}

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
        match _type {
            Primitive::Triangle=>{

            },
            Primitive::Line=>panic!("Drawing primitives other than triangle is not implemented yet!")
        }

        let buf = &self.pos_buf[&pos_buffer.pos_id];
        let ind = &self.ind_buf[&ind_buffer.ind_id];

        let f1 = (100f32-0.1f32)/2.0f32;
        let f2 = (100f32+0.1f32)/2.0f32;

        let mvp = self.projection.unwrap() * self.view.unwrap() * self.model.unwrap();

        let mut t_buf = Vec::new();
        for i in ind.iter() {
            let mut v:[Vector4f;3]=[
                mvp * to_vec4(&buf[i[0]as usize],1.0f32),
                mvp * to_vec4(&buf[i[1]as usize],1.0f32),
                mvp * to_vec4(&buf[i[2]as usize],1.0f32)
            ];

            for vec in v.iter_mut() {
                *vec /= vec.w;
            }

            for vert in v.iter_mut() {
                vert.x = 0.5f32*(self.width as f32)*(vert.x + 1.0f32);
                vert.y = 0.5f32*(self.height as f32)*(vert.y + 1.0f32);
                vert.z = vert.z + f1 + f2;
            }

            let mut t = Triangle::new();
            for ind in 0..3 {
                t.set_vertex(ind,v[ind as usize].truncate());
                t.set_vertex(ind,v[ind as usize].truncate());
                t.set_vertex(ind,v[ind as usize].truncate());
            }

            t.set_color(0,255.0,0.0,0.0);
            t.set_color(1,0.0,255.0,0.0);
            t.set_color(2,0.0,0.0,255.0);

            t_buf.push(t);
        }

        for t in t_buf.iter() {
            self.rasterize_wireframe(t);
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

    fn draw_line(&mut self,begin:Vector3f,end:Vector3f){
        let Vector3f{x:x1,y:y1, .. } = begin;
        let Vector3f { x:x2,y:y2,..} = end;
        let line_color = Vector3f::new(255.0,255.0,255.0);

        let dx = (x2-x1) as i32;
        let dy = (y2-y1) as i32;
        let dx1=dx.abs();
        let dy1=dy.abs();
        let mut  px = 2*dy1-dx1;
        let mut py = 2*dx1-dy1;

        let mut x:i32;
        let mut y:i32;
        if dy1<=dx1{

            let xe:i32;
            if dx>=0{
                x = x1 as i32;
                y = y1 as i32;
                xe = x2 as i32;
            }else {
                x = x2 as i32;
                y = y2 as i32;
                xe = x1 as i32;
            }

            self.set_pixel(&Vector3i::new(x as i32,y as i32,1),line_color);

            while x<xe {
                x+=1;
                if px<0{
                    px = px+2*dy1;
                }else{
                    if (dx<0&&dy<0)||(dx>0&&dy>0) {
                        y+=1;
                    }else{
                        y-=1;
                    }
                    px = px + 2*(dy1-dx1);
                }
                self.set_pixel(&Vector3i::new(x,y,1),line_color);
            }
        }else{
            let ye:i32;
            if dy>=0{
                x = x1 as i32;
                y = y1 as i32;
                ye = y2 as i32;
            }else{
                x = x2 as i32;
                y= y2 as i32;
                ye = y1 as i32;
            }
            self.set_pixel(&Vector3i::new(x as i32,y as i32,1),line_color);

            while y<ye {
                y+=1;
                if py<=0{
                    py=py+2*dx1;
                }else{
                    if (dx<0&&dy<0)||(dx>0&& dy>0){
                        x+=1;
                    }else{
                        x-=1;
                    }
                    py = py+2*(dx1-dy1);
                }
                self.set_pixel(&Vector3i::new(x,y,1),line_color);
            }
        }

    }

    fn rasterize_wireframe(&mut self,t:&Triangle){
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