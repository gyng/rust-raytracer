#![cfg_attr(test, allow(dead_code))]
use ::scene::{Camera, Scene};

pub mod bunny;
pub mod cornell;
pub mod cow;
pub mod easing;
pub mod fresnel;
pub mod heptoroid;
pub mod lucy;
pub mod sibenik;
pub mod sphere;
pub mod sponza;
pub mod tachikoma;
pub mod teapot;

pub trait SceneConfig {
    fn get_camera(&self, image_width: u32, image_height: u32, fov: f64) -> Camera;

    fn get_animation_camera(&self, image_width: u32, image_height: u32, fov: f64) -> Camera {
        self.get_camera(image_width, image_height, fov)
    }

    fn get_scene(&self) -> Scene;
}

pub fn scene_by_name(name: &str) -> Option<Box<SceneConfig>> {
    Some(match name {
        "bunny" => Box::new(bunny::BunnyConfig),
        "cornell" => Box::new(cornell::CornelConfig),
        "cow" => Box::new(cow::CowConfig),
        "easing" => Box::new(easing::EasingConfig),
        "fresnel" => Box::new(fresnel::FresnelConfig),
        "heptoroid-shiny" => Box::new(heptoroid::HeptoroidConfig::shiny()),
        "heptoroid-white" => Box::new(heptoroid::HeptoroidConfig::white()),
        "heptoroid-refractive" => Box::new(heptoroid::HeptoroidConfig::refractive()),
        "lucy" => Box::new(lucy::LucyConfig),
        "sibenik" => Box::new(sibenik::SibenikConfig),
        "sphere" => Box::new(sphere::SphereConfig),
        "sponza" => Box::new(sponza::SponzaConfig),
        "tachikoma" => Box::new(tachikoma::TachikomaConfig),
        "teapot" => Box::new(teapot::TeapotConfig),
        _ => return None,
    })
}