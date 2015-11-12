use rustc_serialize::json;

use voxelblock::VoxelBlock;
use palette::Palette;

#[derive(RustcDecodable, RustcEncodable)]
struct ZoxelModel {
    creator: String,
    height: usize,
    width: usize,
    depth: usize,
    version: usize,
    frame1: Vec<(usize, usize, usize, u32)>
}

pub fn load(data: &str) -> Result<(Palette, VoxelBlock), json::DecoderError> {
    let obj: ZoxelModel = try!(json::decode(data));
    let mut block: VoxelBlock = VoxelBlock::new();
    let mut pal: Palette = Palette::empty();
    for (x, y, z, v) in obj.frame1 {
        block.data[x][y][z] = pal.add_color(
            ((v & 0xFF000000) >> 24) as f32 / 255.0f32,
            ((v & 0x00FF0000) >> 16) as f32 / 255.0f32,
            ((v & 0x0000FF00) >> 8) as f32 / 255.0f32,
            ((v & 0x000000FF) >> 0) as f32 / 255.0f32
        );
    }
    Ok((pal, block))
}
