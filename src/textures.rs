use macroquad::Error;
use macroquad::prelude::*;

pub struct Textures {
    pub meteor: Texture2D,
}

pub async fn load_textures() -> Result<Textures, Error> {
    let meteor = load_texture("./assets/meteor.png").await?;
    Ok(Textures { meteor })
}
