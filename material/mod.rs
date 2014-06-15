pub use self::material::Material;
pub mod material;

pub mod Materials {
    pub use self::cooktorrancematerial::CookTorranceMaterial;
    pub use self::flatmaterial::FlatMaterial;
    pub use self::phongmaterial::PhongMaterial;

    mod cooktorrancematerial;
    mod flatmaterial;
    mod phongmaterial;
}
