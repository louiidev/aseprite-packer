use std::path::Path;

use aseprite_packer::{AsepritePacker, AsepritePackerConfig};

fn main() {
    let config = AsepritePackerConfig {
        path: Path::new("examples/ase_files"),
        output_image_location: Some(Path::new("examples/output/output.png")),
        output_ron_location: Some(Path::new("examples/output/output.ron")),
        ..Default::default()
    };

    let atlas = AsepritePacker::new(config);
    println!("{:?}", atlas.packed_texture_data.get("small_1"));
}
