pub use self::material::Material;
pub use self::texture::Texture;
pub mod material;
pub mod texture;

pub mod materials {
    pub use self::cooktorrancematerial::CookTorranceMaterial;
    pub use self::flatmaterial::FlatMaterial;
    pub use self::phongmaterial::PhongMaterial;

    mod cooktorrancematerial;
    mod flatmaterial;
    mod phongmaterial;
}

pub mod textures {
    pub use self::checkertexture::CheckerTexture;
    pub use self::uvtexture::UVTexture;
    pub use self::imagetexture::ImageTexture;
    pub use self::cubemap::CubeMap;

    mod checkertexture;
    mod uvtexture;
    mod imagetexture;
    mod cubemap;
}
