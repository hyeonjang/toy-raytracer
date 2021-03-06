use rxmath::vector::*;
use rxmath::random::*;

use crate::intersect::ray::*;
use crate::sample::*;

#[derive(Default, Copy, Clone)]
pub struct Camera {
    origin : vec3,
    lower_left_corner : vec3,
    horizontal : vec3,
    vertical : vec3,
    w:vec3, u:vec3, v:vec3,
    lens_radius:f32,
    time0:f32,
    time1:f32,
}

impl Camera {
    pub fn new(lookfrom:vec3, lookat:vec3, vup:vec3, vfov:f32, aspect_ratio:f32, apeture:f32, focus_dist:f32, time0:f32, time1:f32) -> Self {
        let theta = rxmath::degrees_to_radians(vfov);
        let h = f32::tan(theta/2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = normalize(lookfrom-lookat);
        let u = normalize(cross(w, vup));
        let v = cross(w, u);

        let origin = lookfrom;
        let horizontal = u*viewport_width*focus_dist;//u*viewport_width;
        let vertical = v*viewport_height*focus_dist;//v*viewport_height;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - w*focus_dist;
        
        let lens_radius = apeture/2.0;

        return Camera{ origin:origin, horizontal:horizontal, vertical:vertical, lower_left_corner:lower_left_corner, w:w, u:u, v:v, lens_radius:lens_radius, time0:time0, time1:time1 }
    }

    pub fn get_ray(&self, s:f32, t:f32) -> Ray{
        let rd = random_unit_disk()*self.lens_radius;
        let offset = self.u*rd.x + self.v*rd.y;

        return Ray{ 
            o:self.origin+offset, 
            d:(self.lower_left_corner+self.horizontal*s+self.vertical*t-self.origin-offset),
            t_max:f32::MAX,
            tm:random_range_f32(self.time0, self.time1)
        }
    }
} 