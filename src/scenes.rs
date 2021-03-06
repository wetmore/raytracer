use crate::{
    hittable::{HittableList, HittableType},
    materials::MaterialType,
    solids::Sphere,
    texture::TextureType,
    vec::Vec3,
};
use rand::{thread_rng, Rng};

pub enum Scene {
    THREE_BALLS,
    RANDOM_BALLS(u16),
    SHINY,
}

pub fn use_scene(s: Scene) -> HittableList {
    match s {
        Scene::THREE_BALLS => {
            let mut world = HittableList::new();
            let tex =
                TextureType::Checker(0.01, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.12, 0.45, 0.15));

            world.add(
                HittableType::sphere(Vec3::new(0.0, 0.0, -1.0), 0.5),
                MaterialType::Lambertian(tex),
            );
            // ground
            world.add(
                HittableType::sphere(Vec3::new(0.0, -100.5, -1.0), 100.0),
                MaterialType::Metal(Vec3::new(0.8, 0.8, 0.9), 0.1),
            );

            world.add(
                HittableType::sphere(Vec3::new(1.0, 0.0, -1.0), 0.5),
                MaterialType::Metal(Vec3::new(0.40625, 0.1015625, 0.52734375), 0.0),
            );
            // bubble
            world.add(
                HittableType::sphere(Vec3::new(-1.0, 0.0, -1.0), 0.5),
                MaterialType::Dielectric(1.5),
            );
            world.add(
                HittableType::sphere(Vec3::new(-1.0, 0.0, -1.0), -0.45),
                MaterialType::Dielectric(1.5),
            );

            return world;
        }

        Scene::SHINY => {
            let mut world = HittableList::new();
            let tex =
                TextureType::Checker(0.01, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.12, 0.45, 0.15));
            let metal = MaterialType::Metal(Vec3::new(0.9, 0.9, 0.8), 0.0);

            world.add(HittableType::sphere(Vec3::new(0.0, 0.0, -1.0), 0.5), metal);
            // ground
            world.add(
                HittableType::sphere(Vec3::new(0.0, -100.5, -1.0), 100.0),
                metal,
            );

            world.add(HittableType::sphere(Vec3::new(1.0, 0.0, -1.0), 0.5), metal);
            // bubble
            world.add(
                HittableType::sphere(Vec3::new(-1.0, 0.0, -1.0), 0.5),
                MaterialType::Dielectric(1.5),
            );
            world.add(
                HittableType::sphere(Vec3::new(-1.0, 0.0, -1.0), -0.45),
                MaterialType::Dielectric(1.5),
            );

            return world;
        }

        Scene::RANDOM_BALLS(num) => {
            let mut rng = thread_rng();
            let mut world = HittableList::new();

            for _ in 0..num {
                world.add(
                    HittableType::sphere(
                        Vec3::new(
                            rng.gen_range(-100.0, 100.0),
                            rng.gen_range(-100.0, 100.0),
                            rng.gen_range(-200.0, -20.0),
                        ),
                        rng.gen_range(1.0, 20.0),
                    ),
                    MaterialType::Metal(
                        Vec3::new(
                            rng.gen_range(0.0, 1.0),
                            rng.gen_range(0.0, 1.0),
                            rng.gen_range(0.0, 1.0),
                        ),
                        rng.gen_range(0.0, 1.0),
                    ),
                );
            }

            return world;
        }
    }
}
