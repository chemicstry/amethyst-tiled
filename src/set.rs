//! Module to help pack tile sets and convert them into amethyst
//! https://github.com/amethyst/sheep/blob/master/sheep/examples/simple_pack/main.rs
use image::{DynamicImage, GenericImage, GenericImageView, ImageResult, RgbaImage};
use sheep::{pack, InputSprite, SimplePacker, SpriteSheet};
use tiled::Tileset;

pub fn find_then_pack(tiles: &Tileset) -> Option<SpriteSheet> {
    let mut tile_bytes = Vec::with_capacity(tiles.tiles.len());

    for img in tiles.images.iter() {
        let mut img_src = image::open(&img.source).ok()?;
        let img_src = img_src.as_mut_rgba8()?;

        for x in (tiles.margin..img.width as u32 - tiles.margin)
            .step_by((tiles.tile_width + tiles.spacing) as usize)
        {
            for y in (tiles.margin..img.height as u32 - tiles.margin)
                .step_by((tiles.tile_height + tiles.spacing) as usize)
            {
                let tile_pixels = img_src
                    .sub_image(x, y, tiles.tile_width, tiles.tile_height)
                    .pixels()
                    .map(|it| it.2 .0.to_vec())
                    .flat_map(|x| x)
                    .collect();

                let sprite = InputSprite {
                    dimensions: (tiles.tile_width, tiles.tile_height),
                    bytes: tile_pixels,
                };

                tile_bytes.push(sprite);
            }
        }
    }

    // There is guaranteed to be exactly one resulting sprite sheet
    Some(pack::<SimplePacker>(tile_bytes, 4, Default::default()).remove(0))
}
