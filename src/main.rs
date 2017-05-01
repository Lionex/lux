mod vec3;

use vec3::Vec3;

fn main() {
    let (width,height): (u32,u32) = (600,300);
    let w = width as f64;
    let h = height as f64;
    println!("P3\n{} {}\n255\n",width, height);
    for y in 1..height+1 {
        for x in 1..width+1 {

            let r = x as f64;
            let c = y as f64;

            let color = 255. * &Vec3 {
                x: c/h,
                y: r/w,
                z: 0.2,
            };

            println!("{} {} {}\n",color.x as i32,color.y as i32,color.z as i32);
        }
    }
}
