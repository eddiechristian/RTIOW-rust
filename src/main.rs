mod aabb;
mod bvh_node;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod instance;
mod interval;
mod material;
mod onb;
mod pdf;
mod perlin;
mod ray;
mod rtweekend;
mod sphere;
mod texture;
mod triangle;

use std::sync::Arc;

use crate::bvh_node::BVHNode;
use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::instance::{Rotate, Translate};
use crate::material::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use crate::rtweekend::{
    Point3, Vec3, random_double, random_double_range, random_vec3, random_vec3_min_max,
};
use crate::sphere::Sphere;
use crate::texture::{CheckerTexture, NoiseStyle, NoiseTexture, Texture};
use crate::triangle::{add_quad, make_box, make_prism, make_pyramid};

fn bouncing_spheres() -> std::io::Result<()> {
    // World
    let mut world = HittableList::new();

    let checker: Arc<dyn Texture> = Arc::new(CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    world.add(Box::new(Sphere::new(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::from_texture(checker)),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                let sphere_material: Arc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = random_vec3().component_mul(&random_vec3());
                    Arc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_vec3_min_max(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    // glass
                    Arc::new(Dielectric::new(1.5))
                };
                world.add(Box::new(Sphere::new(&center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        &Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        &Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        &Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let world = BVHNode::from_list(world);

    // Camera
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world, None)
}

fn checkered_spheres() -> std::io::Result<()> {
    // World
    let mut world = HittableList::new();

    let checker: Arc<dyn Texture> = Arc::new(CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    world.add(Box::new(Sphere::new(
        &Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::from_texture(Arc::clone(&checker))),
    )));
    world.add(Box::new(Sphere::new(
        &Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::from_texture(checker)),
    )));

    let world = BVHNode::from_list(world);

    // Camera
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world, None)
}

fn perlin_spheres() -> std::io::Result<()> {
    // World
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    // Three balls side by side along z, one per noise style. The camera looks
    // down -x, so +z is on the left of the image.
    let styles = [
        // Smooth Perlin noise (book Image 13).
        (2.4, NoiseTexture::new(4.0, NoiseStyle::Smooth)),
        // Turbulence used directly, camouflage netting (book Image 14).
        (0.0, NoiseTexture::new(1.0, NoiseStyle::Turbulence)),
        // Marble (book Image 15).
        (-2.4, NoiseTexture::new(4.0, NoiseStyle::Marble)),
    ];
    for (z, tex) in styles {
        world.add(Box::new(Sphere::new(
            &Point3::new(0.0, 1.0, z),
            1.0,
            Arc::new(Lambertian::from_texture(Arc::new(tex))),
        )));
    }

    let world = BVHNode::from_list(world);

    // Camera
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 25.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 0.0);
    cam.lookat = Point3::new(0.0, 1.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world, None)
}

fn cornell_box() -> std::io::Result<()> {
    // World
    let mut world = HittableList::new();

    let red: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light: Arc<dyn Material> = Arc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));

    // Walls and ceiling light, each quad made of two triangles.
    let p = Point3::new;
    let v = Vec3::new;
    add_quad(
        &mut world,
        p(555.0, 0.0, 0.0),
        v(0.0, 555.0, 0.0),
        v(0.0, 0.0, 555.0),
        green,
    );
    add_quad(
        &mut world,
        p(0.0, 0.0, 0.0),
        v(0.0, 555.0, 0.0),
        v(0.0, 0.0, 555.0),
        red,
    );
    add_quad(
        &mut world,
        p(343.0, 554.0, 332.0),
        v(-130.0, 0.0, 0.0),
        v(0.0, 0.0, -105.0),
        light,
    );
    add_quad(
        &mut world,
        p(0.0, 0.0, 0.0),
        v(555.0, 0.0, 0.0),
        v(0.0, 0.0, 555.0),
        Arc::clone(&white),
    );
    add_quad(
        &mut world,
        p(555.0, 555.0, 555.0),
        v(-555.0, 0.0, 0.0),
        v(0.0, 0.0, -555.0),
        Arc::clone(&white),
    );
    add_quad(
        &mut world,
        p(0.0, 0.0, 555.0),
        v(555.0, 0.0, 0.0),
        v(0.0, 555.0, 0.0),
        Arc::clone(&white),
    );

    // Two boxes, built at the origin, then rotated about y and moved into place.
    let box1: Arc<dyn Hittable> = Arc::new(make_box(
        p(0.0, 0.0, 0.0),
        p(165.0, 330.0, 165.0),
        Arc::clone(&white),
    ));
    let box1: Arc<dyn Hittable> = Arc::new(Rotate::new(box1, Vec3::y(), 15.0));
    world.add(Box::new(Translate::new(box1, v(265.0, 0.0, 295.0))));

    let box2: Arc<dyn Hittable> =
        Arc::new(make_box(p(0.0, 0.0, 0.0), p(165.0, 165.0, 165.0), white));
    let box2: Arc<dyn Hittable> = Arc::new(Rotate::new(box2, Vec3::y(), -18.0));
    world.add(Box::new(Translate::new(box2, v(130.0, 0.0, 65.0))));

    let world = BVHNode::from_list(world);

    // Camera
    let mut cam = Camera::new();
    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 200;
    cam.max_depth = 50;
    cam.background = Some(Color::new(0.0, 0.0, 0.0));

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world, None)
}

// Test scene for triangle shapes and instances. Every shape is built around the
// origin, rotated there, then translated into place.
fn shapes() -> std::io::Result<()> {
    // World
    let mut world = HittableList::new();

    let checker: Arc<dyn Texture> = Arc::new(CheckerTexture::from_colors(
        1.0,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    world.add(Box::new(Sphere::new(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::from_texture(checker)),
    )));

    let red: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.7, 0.15, 0.1)));
    let blue: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.1, 0.25, 0.7)));
    let yellow: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.8, 0.65, 0.1)));
    let purple: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.45, 0.15, 0.6)));
    let metal: Arc<dyn Material> = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.05));

    let origin = Point3::origin();
    let pyramid = |mat: &Arc<dyn Material>| -> Arc<dyn Hittable> {
        Arc::new(make_pyramid(origin, 1.6, 1.6, Arc::clone(mat)))
    };
    let prism = |mat: &Arc<dyn Material>| -> Arc<dyn Hittable> {
        Arc::new(make_prism(origin, 1.6, 1.4, 1.6, Arc::clone(mat)))
    };
    let rotate =
        |obj, axis, degrees| -> Arc<dyn Hittable> { Arc::new(Rotate::new(obj, axis, degrees)) };
    let place =
        |world: &mut HittableList, obj, offset| world.add(Box::new(Translate::new(obj, offset)));

    // Back row: the same prism rotated 45° about each single axis.
    place(
        &mut world,
        rotate(prism(&blue), Vec3::x(), 45.0),
        Vec3::new(-3.0, 1.3, -3.0),
    );
    place(
        &mut world,
        rotate(prism(&blue), Vec3::y(), 45.0),
        Vec3::new(0.0, 0.8, -3.0),
    );
    place(
        &mut world,
        rotate(prism(&blue), Vec3::z(), 45.0),
        Vec3::new(3.0, 1.2, -3.0),
    );

    // Front row, left to right:
    // 1. Reference pyramid: no rotation, translated along x only.
    place(&mut world, pyramid(&red), Vec3::new(-4.5, 0.0, 0.0));

    // 2. Pyramid rotated 40° about the diagonal axis (1,1,1).
    place(
        &mut world,
        rotate(pyramid(&yellow), Vec3::new(1.0, 1.0, 1.0), 40.0),
        Vec3::new(-1.5, 0.65, 1.0),
    );

    // 3. Prism rotated about all three axes in turn: x, then y, then z.
    let all_axes = rotate(
        rotate(rotate(prism(&purple), Vec3::x(), 30.0), Vec3::y(), 45.0),
        Vec3::z(),
        20.0,
    );
    place(&mut world, all_axes, Vec3::new(1.5, 1.1, 1.0));

    // 4. Metal box rotated about the axis (1,0,1), translated along all three axes so it floats.
    let cube: Arc<dyn Hittable> = Arc::new(make_box(
        Point3::new(-0.7, -0.7, -0.7),
        Point3::new(0.7, 0.7, 0.7),
        Arc::clone(&metal),
    ));
    place(
        &mut world,
        rotate(cube, Vec3::new(1.0, 0.0, 1.0), 35.0),
        Vec3::new(4.5, 2.0, 0.5),
    );

    let world = BVHNode::from_list(world);

    // Camera
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 35.0;
    cam.lookfrom = Point3::new(0.0, 5.0, 13.0);
    cam.lookat = Point3::new(0.0, 1.0, -0.5);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world, None)
}

// Book 3's final Cornell box: importance-sampled toward the ceiling light and
// the glass sphere, so 1000 samples come out far less noisy than Book 2's.
fn cornell_box_sampled() -> std::io::Result<()> {
    // World
    let mut world = HittableList::new();

    let red: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light: Arc<dyn Material> = Arc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));

    // Walls, and a ceiling light whose front face (u × v) points down into the room.
    let p = Point3::new;
    let v = Vec3::new;
    let light_corner = p(343.0, 554.0, 332.0);
    let (light_u, light_v) = (v(-130.0, 0.0, 0.0), v(0.0, 0.0, -105.0));
    add_quad(
        &mut world,
        p(555.0, 0.0, 0.0),
        v(0.0, 555.0, 0.0),
        v(0.0, 0.0, 555.0),
        green,
    );
    add_quad(
        &mut world,
        p(0.0, 0.0, 0.0),
        v(0.0, 555.0, 0.0),
        v(0.0, 0.0, 555.0),
        red,
    );
    add_quad(
        &mut world,
        light_corner,
        light_u,
        light_v,
        Arc::clone(&light),
    );
    add_quad(
        &mut world,
        p(0.0, 0.0, 0.0),
        v(555.0, 0.0, 0.0),
        v(0.0, 0.0, 555.0),
        Arc::clone(&white),
    );
    add_quad(
        &mut world,
        p(555.0, 555.0, 555.0),
        v(-555.0, 0.0, 0.0),
        v(0.0, 0.0, -555.0),
        Arc::clone(&white),
    );
    add_quad(
        &mut world,
        p(0.0, 0.0, 555.0),
        v(555.0, 0.0, 0.0),
        v(0.0, 555.0, 0.0),
        Arc::clone(&white),
    );

    // Box
    let box1: Arc<dyn Hittable> =
        Arc::new(make_box(p(0.0, 0.0, 0.0), p(165.0, 330.0, 165.0), white));
    let box1: Arc<dyn Hittable> = Arc::new(Rotate::new(box1, Vec3::y(), 15.0));
    world.add(Box::new(Translate::new(box1, v(265.0, 0.0, 295.0))));

    // Glass sphere
    let glass: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
    let sphere_center = p(190.0, 90.0, 190.0);
    world.add(Box::new(Sphere::new(&sphere_center, 90.0, glass)));

    let world = BVHNode::from_list(world);

    // Objects to aim extra rays at: the same light (two triangles) and glass sphere.
    // Only their geometry is used, so the material is a placeholder (the book passes none).
    let mut lights = HittableList::new();
    add_quad(
        &mut lights,
        light_corner,
        light_u,
        light_v,
        Arc::clone(&light),
    );
    lights.add(Box::new(Sphere::new(&sphere_center, 90.0, light)));

    // Camera
    let mut cam = Camera::new();
    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 1000;
    cam.max_depth = 50;
    cam.background = Some(Color::new(0.0, 0.0, 0.0));

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world, Some(&lights))
}

fn main() -> std::io::Result<()> {
    // Pick the scene to render.
    match 6 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => perlin_spheres(),
        4 => cornell_box(),
        5 => shapes(),
        6 => cornell_box_sampled(),
        _ => unreachable!(),
    }
}
