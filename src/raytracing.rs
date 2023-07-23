use crate::graphics::*;

const VIEWPORT_WIDTH: f32 = 1.0;
const VIEWPORT_HEIGHT: f32 = 1.0;
const VIEWPORT_DIST: f32 = 1.0;

pub struct Sphere {
    pos: Point<f32>,
    radius: f32,
    colour: CanvasColour
}

impl Sphere {
    pub fn new(pos: Point<f32>, radius: f32, colour: CanvasColour) -> Self {
        Self {
            pos: pos,
            radius: radius,
            colour: colour
        }
    }
}

pub struct Scene {
    objects: Vec<Sphere>
}

impl Scene {
    pub fn init() -> Scene {
        Scene {
            objects: vec![
                Sphere::new(Point::<f32>::new(0.0, -1.0, 3.0), 4.0, CanvasColour::from((255, 0, 0))),
                Sphere::new(Point::<f32>::new(1.0, -0.5, 2.0), 3.0, CanvasColour::from((0, 255, 0))),
                Sphere::new(Point::<f32>::new(1.0, -0.5, 2.0), 3.0, CanvasColour::from((0, 0, 255))),
            ]
        }
    }
}

pub fn update(screen: &mut PixelBuf, scene: &Scene) {

    let camera = Point::<f32>::new(0.0, 0.0, 0.0);

    for x in -CANVAS_WIDTH / 2 .. CANVAS_WIDTH / 2 {
        for y in -CANVAS_HEIGHT / 2 .. CANVAS_HEIGHT / 2 {

            let ray_dir = canvas_to_viewport(x, y);
            let colour = trace_ray(camera, &scene, ray_dir, 1.0, f32::INFINITY);

            put_pixel(screen, x, y, colour);

        }
    }

}

/*
checks if the ray hit any spheres and returns the colour of the sphere closest to the camera 
(or a background colour if the ray didn't hit anything)
*/
fn trace_ray(camera: Point<f32>, scene: &Scene, dir: Point<f32>, t_min: f32, t_max: f32) -> CanvasColour {

    let mut closest_t = f32::INFINITY;
    let mut closest_sphere: Option<&Sphere> = None;

    for sphere in scene.objects.iter() {
        let (t1, t2) = intersect_ray_sphere(camera, dir, sphere);

        if (t1 > t_min && t1 < t_max) && t1 < closest_t {
            closest_t = t1;
            closest_sphere = Some(&sphere);
        }

        if (t2 >= t_min && t2 <= t_max) && t2 < closest_t {
            closest_t = t2;
            closest_sphere = Some(&sphere);
        }


    }

    // if the ray didn't hit any sphere
    if closest_sphere.is_none() {
        return CanvasColour::from((255, 255, 255));
    }

    return closest_sphere.unwrap().colour;

}

/*
Solves the quadratic equation
*/
fn intersect_ray_sphere(camera: Point<f32>, dir: Point<f32>, sphere: &Sphere) -> (f32, f32) {

    let r = sphere.radius;
    let co = camera - sphere.pos;

    let a = dir.dot(dir);
    let b = co.dot(dir) * 2.0;
    let c = co.dot(co) - r*r;

    let descriminant = b * b - 4.0 * a * c;
    if descriminant < 0.0 {
        return (f32::INFINITY, f32::INFINITY);
    }

    let t1 = (-b + descriminant.sqrt()) / (2.0 * a);
    let t2 = (-b - descriminant.sqrt()) / (2.0 * a);

    (t1, t2)

}

fn canvas_to_viewport(cx: i32, cy: i32) -> Point<f32> {
    Point::<f32>::new(cx as f32 * VIEWPORT_WIDTH / CANVAS_WIDTH as f32, cy as f32 * VIEWPORT_HEIGHT / CANVAS_HEIGHT as f32, VIEWPORT_DIST)
}