use rxmath::vector::*;

use crate::intersect::*;

pub trait Union<T>{
    type Output;
    fn bounds(b0:T, b1:T) -> Self::Output;
    fn expand(&mut self, b:T);
}

impl Union<Bounds> for Bounds {
    type Output = Self;
    fn bounds(c0:Bounds, c1:Bounds) -> Self::Output {
        let min = vec3(f32::min(c0.min.x, c1.min.x), f32::min(c0.min.y, c1.min.y), f32::min(c0.min.z, c1.min.z));
        let max = vec3(f32::max(c0.max.x, c1.max.x), f32::max(c0.max.y, c1.max.y), f32::max(c0.max.z, c1.max.z));
        return Bounds::new(min, max);
    }
    fn expand(&mut self, b:Bounds){
        self.min = vec3(f32::min(self.min.x, b.min.x), f32::min(self.min.x, b.min.x), f32::min(self.min.x, b.min.x));
        self.max = vec3(f32::max(self.max.x, b.max.x), f32::max(self.min.x, b.min.x), f32::max(self.min.x, b.min.x));
    }
}

impl Union<vec3> for Bounds {
    type Output = Self;
    fn bounds(c0:vec3, c1:vec3) -> Self::Output {
        let min = vec3(f32::min(c0.x, c1.x), f32::min(c0.y, c1.y), f32::min(c0.z, c1.z));
        let max = vec3(f32::max(c0.x, c1.x), f32::max(c0.y, c1.y), f32::max(c0.z, c1.z));
        return Bounds::new(min, max);
    }
    fn expand(&mut self, b:vec3) {
        self.min = vec3(f32::min(self.min.x, b.x), f32::min(self.min.x, b.x), f32::min(self.min.x, b.x));
        self.max = vec3(f32::max(self.max.x, b.x), f32::max(self.min.x, b.x), f32::max(self.min.x, b.x));
    }
}

#[derive(Copy, Clone, Default)]
pub struct Bounds {
    pub min:vec3,
    pub max:vec3,
}

impl Bounds {
    pub fn new(min:vec3, max:vec3) -> Self {
        Bounds { min:min, max:max }
    }
    
    pub fn default() -> Self {
        Bounds { min:vec3::min(), max:vec3::max() }
    }

    pub fn size(&self) -> vec3 {
        return self.max - self.min;
    }

    pub fn max_extend(&self) -> i32 {
        let e = self.size();
        return if e.x>e.y && e.x>e.z { 0 } else if e.y>e.z { 1 } else { 2 };
    }

    // intersection 
    // optimized code by Andrew Kensler at Pixar
    pub fn hit(r:ray, mut t_min:f32, mut t_max:f32) -> bool {
        for i in 0..3 {
            let inv_d = 1.0 / r.dir[i];
            let mut t0 = (vec3::min()[i] - r.o[i]) * inv_d;
            let mut t1 = (vec3::max()[i] - r.o[i]) * inv_d;
            if inv_d<0.0 { std::mem::swap(&mut t0, &mut t1); }
            t_min = if t0>t_min { t0 } else { t_min };
            t_max = if t1<t_max { t1 } else { t_max };
            if t_max<=t_min { return false }
        }
        return true;
    }
}