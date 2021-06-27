use asefile::AsepriteFile;
use image::DynamicImage;
use std::{collections::HashMap, convert::TryInto, fs::File, path::Path};
use texture_packer::{exporter::ImageExporter, TexturePacker, TexturePackerConfig};

#[derive(Debug)]
pub struct AseTextureData {
    pub width: u32,
    pub height: u32,
    pub basename: String,
    pub frame: u32,
    pub x: u32,
    pub y: u32,
}

#[derive(Debug)]
pub struct AsePackerData {
    pub image: DynamicImage,
    pub packed_texture_data: HashMap<String, AseTextureData>,
}

pub fn pack_ase_data<'a>(names: &[&str], path: Option<&Path>) -> AsePackerData {
    let config = TexturePackerConfig {
        max_width: std::u32::MAX,
        max_height: std::u32::MAX,
        allow_rotation: false,
        texture_outlines: false,
        border_padding: 0,
        texture_padding: 0,
        ..Default::default()
    };

    let mut packer = TexturePacker::new_skyline(config);

    let mut packed_texture_data: HashMap<String, AseTextureData> = HashMap::default();

    for basename in names.iter() {
        let name = format!("{}.aseprite", basename);
        let p = if let Some(path) = path {
            let _path = path.clone();
            _path.join(name)
        } else {
            Path::new(&name).try_into().unwrap()
        };

        // let path = env::current_dir().unwrap();
        let f = File::open(p).unwrap();
        let ase_file = AsepriteFile::read(&f);
        match ase_file {
            Err(e) => println!("error is: {}", e),
            Ok(ase) => {
                for frame in 0..ase.num_frames() {
                    ase.width();
                    let key: String = format!("{}_{}", basename, frame);
                    packer.pack_own(key.clone(), ase.frame(frame).image());
                    let frame_data = packer.get_frame(&key).unwrap();
                    let source = frame_data.frame;
                    packed_texture_data.insert(
                        key.clone(),
                        AseTextureData {
                            width: source.w,
                            height: source.h,
                            x: source.x,
                            y: source.y,
                            basename: basename.to_string(),
                            frame,
                        },
                    );
                }
            }
        }
    }

    let image = ImageExporter::export(&packer).unwrap();

    AsePackerData {
        image,
        packed_texture_data,
    }
}
