use crate::graphics::*;

const VIEWPORT_WIDTH: f32 = 1.0;
const VIEWPORT_HEIGHT: f32 = 1.0;
const VIEWPORT_DIST: f32 = 1.0;

#[derive(PartialEq)]
enum LightType {
    Point,
    Directional,
    Ambient
}

pub struct Sphere {
    pos: Point<f32>,
    radius: f32,
    colour: CanvasColour,
    #[allow(dead_code)]
    name: String
}

impl Sphere {
    pub fn new(pos: Point<f32>, radius: f32, colour: CanvasColour, name: String) -> Self {
        Self {
            pos: pos,
            radius: radius,
            colour: colour,
            name: name
        }
    }
}

pub struct Light {
    light_type: LightType,
    intensity: f32,
    pos: Option<Point<f32>>,
    dir: Option<Point<f32>>
}

impl Light {
    fn new(light_type: LightType, intensity: f32, pos: Option<Point<f32>>, dir: Option<Point<f32>>) -> Self {
        Self {
            light_type: light_type,
            intensity: intensity,
            pos: pos,
            dir: dir
        }
    }
}

pub struct Scene {
    objects: Vec<Sphere>,
    lights: Vec<Light>
}

impl Scene {
    pub fn init() -> Scene {
        Scene {
            objects: vec![
                Sphere::new(Point::<f32>::new(0.0, -1.0, 3.0), 1.0, CanvasColour::new(255, 0, 0), "sphere1".to_string()),
                Sphere::new(Point::<f32>::new(2.0, 0.0, 4.0), 1.0, CanvasColour::new(0, 255, 0), "sphere2".to_string()),
                Sphere::new(Point::<f32>::new(-2.0, 0.0, 4.0), 1.0, CanvasColour::new(0, 0, 255), "sphere3".to_string()),
                Sphere::new(Point::<f32>::new(0.0, -5001.0, 0.0), 5000.0, CanvasColour::new(255, 255, 0), "sphere3".to_string()),
            ],
            lights: vec![
                Light::new(LightType::Ambient, 0.2, None, None),
                Light::new(LightType::Point, 0.6, Some(Point::<f32>::new(2.0, 1.0, 0.0),), None),
                Light::new(LightType::Directional, 0.2, None, Some(Point::<f32>::new(1.0, 4.0, 4.0)))
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

            //println!("{:?}", colour);

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

        if (t1 >= t_min && t1 <= t_max) && t1 < closest_t {
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
        return CanvasColour::WHITE;
    }

    //println!("hit sphere {} (colour {:?})", closest_sphere.unwrap().name, closest_sphere.unwrap().colour);

    let point = camera + dir.mul_scalar(&closest_t);
    let mut normal = point - closest_sphere.unwrap().pos;
    normal = normal / (normal.dot(normal));

    let lighting_change = compute_lighting(scene, point, normal);

    //println!("lighting result: {}, resulting colour: {:?}", lighting_change, closest_sphere.unwrap().colour * lighting_change);

    // hit sphere
    return closest_sphere.unwrap().colour * lighting_change;

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

    let descriminant = b*b - 4.0*a*c;
    if descriminant < 0.0 {
        return (f32::INFINITY, f32::INFINITY);
    }

    let t1 = (-b + descriminant.sqrt()) / (2.0 * a);
    let t2 = (-b - descriminant.sqrt()) / (2.0 * a);

    (t1, t2)

}

/*
Computes Lighting for the given point using the diffuse reflection equation
*/
fn compute_lighting(scene: &Scene, point: Point<f32>, normal: Point<f32>) -> f32 {
    let mut intensity: f32 = 0.0;

    for light in scene.lights.iter() {

        if light.light_type == LightType::Ambient {
            intensity += light.intensity;
            //println!("intensity: {}", intensity);
        } else {

            let light_dir: Point<f32> = match light.light_type {
                LightType::Point => light.pos.unwrap() - point,
                LightType::Directional => light.dir.unwrap(),
                _ => panic!("Incorrect lighting info")
            };

            let normal_dot_light = normal.dot(light_dir);

            if normal_dot_light > 0.0 {
                intensity += light.intensity * normal_dot_light / (normal.len() * light_dir.len());
            }

        }

    }

    //println!("{}", intensity);

    intensity

}

fn canvas_to_viewport(cx: i32, cy: i32) -> Point<f32> {
    Point::<f32>::new(cx as f32 * VIEWPORT_WIDTH / CANVAS_WIDTH as f32, cy as f32 * VIEWPORT_HEIGHT / CANVAS_HEIGHT as f32, VIEWPORT_DIST)
}