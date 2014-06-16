pub use self::material::Material;
pub mod material;

pub mod materials {
    pub use self::cooktorrancematerial::CookTorranceMaterial;
    pub use self::flatmaterial::FlatMaterial;
    pub use self::phongmaterial::PhongMaterial;

    mod cooktorrancematerial;
    mod flatmaterial;
    mod phongmaterial;
}
