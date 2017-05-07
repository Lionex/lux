mod ray;
mod vec3;

use vec3::Vec3;
use ray::*;

fn color(r: &Ray) -> Vec3 {
    let direction = r.direction();
    let t = 0.5 * (direction.y + 1.);
    &((1.-t)*&Vec3 {x:1., y:1., z:1.}) + &(t*&Vec3 {x:0.5, y:0.7, z:1.})
}

fn main() {
    let (width,height): (u32,u32) = (900,600);
    let w = width as f64;
    let h = height as f64;

    // Define camera as a group of vectors
    let lower_left = Vec3 {x: -2., y: -1., z: -1.};
    let horizontal = Vec3 {x: 4., y: 0., z: 0.};
    let vertical   = Vec3 {x: 0., y: 2., z: 0.};
    let origin     = Vec3::zero();

    println!("P3\n{} {}\n255\n",width, height);
    for y in (1..height+1).map(|x| height - x) {
        for x in 1..width+1 {

            let u = x as f64 / w;
            let v = y as f64 / h;

            let ray = Ray::new(
                origin,
                &lower_left + &(&(u*&horizontal) + &(v*&vertical)),
            );

            let color = 255.99*&color(&ray);

            println!("{} {} {}\n",color.x as i32,color.y as i32,color.z as i32);
        }
    }
}
