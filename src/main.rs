use image::{ImageBuffer, RgbImage};

mod color;
mod light;
mod material;
mod ray;
mod sphere;
mod vector;

use color::Color;
use light::Light;
use material::Material;
use rand::{thread_rng, Rng};
use ray::Ray;
use sphere::Sphere;
use vector::Vec3;

fn main() {
    let width = 2048;
    let height = 1080;
    let mut image: RgbImage = ImageBuffer::new(width, height);
    for (_, _, pixel) in image.enumerate_pixels_mut() {
        *pixel = image::Rgb([0xe6, 0xaf, 0x2e]);
    }

    let mut objects: Vec<Sphere> = vec![];
    let mut rng = thread_rng();
    for _ in 0..200 {
        objects.push(Sphere {
            center: Vec3 {
                x: rng.gen_range(0.0..width as f64),
                y: rng.gen_range(0.0..height as f64),
                z: rng.gen_range(0.0..1000.0),
            },
            radius: rng.gen_range(0.0..50.0),
            material: Material {
                albedo: Color {
                    r: rng.gen_range(0..255),
                    g: rng.gen_range(0..255),
                    b: rng.gen_range(0..255),
                    alpha: 255,
                },
            },
        });
    }

    let light = Light {
        origin: Vec3 {
            x: (width / 2) as f64,
            y: (height / 2) as f64,
            z: 0.0,
        },
        brightness: 1.0,
    };

    for i in 0..width {
        for j in 0..height {
            let ray = Ray {
                origin: Vec3 {
                    x: i as f64,
                    y: j as f64,
                    z: 0.0,
                },
                direction: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
            };
            for object in &objects {
                let point = ray.get_intersection_point(&object);
                match point {
                    Some(vec) => {
                        let other_objects: Vec<_> =
                            objects.iter().filter(|&x| *x != *object).cloned().collect();
                        let color = object.get_color_at(&vec, &light, &other_objects);

                        image.put_pixel(i, j, image::Rgb([color.r, color.g, color.b]));
                    }
                    None => {}
                }
            }
        }
    }

    image.save("test.png").unwrap();
}

#[test]
fn intersection() {
    let ray = Ray {
        origin: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        direction: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };

    let sphere = Sphere {
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 5.0,
        },
        radius: 1.0,
        material: Material {
            albedo: Color {
                r: 0,
                g: 0,
                b: 0,
                alpha: 255,
            },
        },
    };

    assert_eq!(sphere.intersect(&ray).unwrap(), 4.0)
}

#[test]
fn no_intersection() {
    let ray = Ray {
        origin: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        direction: Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
    };

    let sphere = Sphere {
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 5.0,
        },
        radius: 1.0,
        material: Material {
            albedo: Color {
                r: 0,
                g: 0,
                b: 0,
                alpha: 255,
            },
        },
    };

    assert_eq!(sphere.intersect(&ray), None)
}

#[test]
fn intersection_point() {
    let ray = Ray {
        origin: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        direction: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };

    let sphere = Sphere {
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 5.0,
        },
        radius: 1.0,
        material: Material {
            albedo: Color {
                r: 0,
                g: 0,
                b: 0,
                alpha: 255,
            },
        },
    };

    assert_eq!(
        ray.get_intersection_point(&sphere),
        Some(Vec3 {
            x: 0.0,
            y: 0.0,
            z: 4.0
        })
    )
}

#[test]
fn normal_vector_on_sphere_suface() {
    let ray = Ray {
        origin: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        direction: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };

    let sphere = Sphere {
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 5.0,
        },
        radius: 1.0,
        material: Material {
            albedo: Color {
                r: 0,
                g: 0,
                b: 0,
                alpha: 255,
            },
        },
    };

    let intersection_point = ray.get_intersection_point(&sphere);
    assert_eq!(
        intersection_point.unwrap(),
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 4.0
        }
    );
    assert_eq!(
        sphere.get_normal_vec_at(&intersection_point.unwrap()),
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0
        }
    )
}

#[test]
fn direction_between_two_points() {
    let p1 = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 0.0,
    };
    let p2 = Vec3 {
        x: -1.0,
        y: -1.0,
        z: 0.0,
    };

    assert_eq!(
        p1.get_direction_to(&p2),
        Vec3 {
            x: -0.7071067811865475,
            y: -0.7071067811865475,
            z: 0.0
        }
    )
}

#[test]
fn when_light_is_in_front_of_object() {
    let light = Light {
        origin: Vec3 {
            x: 5.0,
            y: 0.0,
            z: 5.0,
        },
        brightness: 1.0,
    };

    let sphere = Sphere {
        center: Vec3 {
            x: -0.7,
            y: 0.0,
            z: 5.0,
        },
        radius: 1.0,
        material: Material {
            albedo: Color {
                r: 0,
                g: 0,
                b: 0,
                alpha: 255,
            },
        },
    };
    let ray = Ray {
        origin: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        direction: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };

    let intersection_point = ray.get_intersection_point(&sphere);
    let relative_obj_position_to_light = light
        .origin
        .get_direction_to(&intersection_point.unwrap())
        .dot(&sphere.get_normal_vec_at(&intersection_point.unwrap()));
    assert!(
        relative_obj_position_to_light > 0.0,
        "relative_obj_position_to_light: {}",
        relative_obj_position_to_light
    )
}

#[test]
fn when_light_is_behind_the_object() {
    let light = Light {
        origin: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 10.0,
        },
        brightness: 1.0,
    };

    let sphere = Sphere {
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 5.0,
        },
        radius: 1.0,
        material: Material {
            albedo: Color {
                r: 0,
                g: 0,
                b: 0,
                alpha: 255,
            },
        },
    };
    let ray = Ray {
        origin: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        direction: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };

    let intersection_point = ray.get_intersection_point(&sphere);
    assert_eq!(
        intersection_point.unwrap(),
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 4.0
        }
    );

    let relative_obj_position_to_light = sphere.get_brightness_at(
        &intersection_point.unwrap(),
        &light,
        &(vec![] as Vec<Sphere>),
    );
    assert!(
        relative_obj_position_to_light.unwrap() == 0.0,
        "relative_obj_position_to_light: {:?}",
        relative_obj_position_to_light
    )
}

#[test]
fn light_is_on_the_side() {
    let light = Light {
        origin: Vec3 {
            x: 5.0,
            y: 0.0,
            z: 3.0,
        },
        brightness: 1.0,
    };

    let sphere = Sphere {
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 5.0,
        },
        radius: 1.0,
        material: Material {
            albedo: Color {
                r: 0,
                g: 0,
                b: 0,
                alpha: 255,
            },
        },
    };
    let ray = Ray {
        origin: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        direction: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };

    let intersection_point = ray.get_intersection_point(&sphere);
    let relative_obj_position_to_light = sphere.get_brightness_at(
        &intersection_point.unwrap(),
        &light,
        &(vec![] as Vec<Sphere>),
    );
    assert!(
        relative_obj_position_to_light.unwrap() > 0.0,
        "relative_obj_position_to_light: {:?}",
        relative_obj_position_to_light
    )
}

#[test]
fn object_casting_shadow() {
    let light = Light {
        origin: Vec3 {
            x: 5.0,
            y: 0.0,
            z: 3.0,
        },
        brightness: 1.0,
    };

    let sphere = Sphere {
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 5.0,
        },
        radius: 1.0,
        material: Material {
            albedo: Color {
                r: 0,
                g: 0,
                b: 0,
                alpha: 255,
            },
        },
    };
    let other_sphere = Sphere {
        center: Vec3 {
            x: 4.0,
            y: 0.0,
            z: 3.5,
        },
        radius: 1.0,
        material: Material {
            albedo: Color {
                r: 0,
                g: 0,
                b: 0,
                alpha: 255,
            },
        },
    };

    let ray = Ray {
        origin: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        direction: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };

    let objects: Vec<Sphere> = vec![other_sphere];

    let intersection_point = ray.get_intersection_point(&sphere);
    let color_at_pixel = sphere.get_color_at(&intersection_point.unwrap(), &light, &objects);
    assert_eq!(
        color_at_pixel,
        Color {
            r: 0,
            g: 0,
            b: 0,
            alpha: 255
        },
        "color_at_pixel: {:?}",
        color_at_pixel
    )
}
