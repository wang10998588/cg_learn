type Vector3f = cgmath::Vector3<f32>;
type Vector2f = cgmath::Vector2<f32>;
type Vector4f = cgmath::Vector4<f32>;

pub struct Triangle{
    pub v:[Vector3f;3],
    pub color:[Vector3f;3],
    pub tex_coords:[Vector2f;3],
    pub normal:[Vector3f;3]
}

impl Triangle{

    pub fn new()->Triangle{
        Triangle{
            v:[Vector3f::new(0f32,0f32,0f32);3],
            color:[Vector3f::new(0f32,0f32,0f32);3],
            tex_coords: [Vector2f::new(0f32,0f32);3],
            normal: [Vector3f::new(0f32,0f32,0f32);3]
        }
    }

    pub fn a(&self)->Vector3f{
        self.v[0]
    }
    pub fn b(&self)->Vector3f{
        self.v[1]
    }
    pub fn c(&self)->Vector3f{
        self.v[2]
    }

    pub fn set_vertex(&mut self, ind:i32, ver:Vector3f){
        self.v[ind] = ver;
    }

    pub fn set_normal(&mut self, ind:i32, n:Vector3f){
        self.normal[ind] = n;
    }

    pub fn set_color(&mut self, ind:i32, r:f32, g:f32, b:f32){
        if r<0.0||r>255.0||g<0.0||g>255.0||b<0.0||b>255.0{
            panic!("Invalid color values");
        }
        self.color[ind] = Vector3f::new(r/255f32,g/255f32,b/255f32);
    }

    pub fn set_tex_coord(&mut self, ind:i32, s:f32, t:f32){
        self.tex_coords[ind] = Vector2f::new(s,t);
    }

    pub fn to_vector4(&self) ->[Vector4f;3]{
        let mut ret:[Vector4f;3] = [Vector4f::new(0f32,0f32,0f32,1f32);3];
        let ind = 0;
        for vec in self.v.iter() {
            let vec4 = &mut ret[ind];
            vec4.x = vec.x;
            vec4.y = vec.y;
            vec4.z = vec.z;
        }
        ret
    }
}