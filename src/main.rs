mod packer;

use std::path::Path;

use packer::{AsepritePackerConfig, AsespritePacker};

fn main() {
    let config = AsepritePackerConfig {
        path: Path::new("src/ase_files"),
        asesprite_file_names: &["big", "small"],
        output_image_location: Some(Path::new("src/output.png")),
        output_ron_location: Some(Path::new("src/output.ron"))
    };

    println!("{:?}", config);

    let atlas = AsespritePacker::new(config);
    println!("{:?}", atlas.packed_texture_data.get("small_1"));

}
