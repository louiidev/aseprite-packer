use asefile::{AsepriteFile, AsepriteParseError};
use image::{DynamicImage, ImageBuffer, Rgba};
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, path::Path};
use std::{fs, io::Write, path::PathBuf};
use texture_packer::{exporter::ImageExporter, TexturePacker, TexturePackerConfig};

#[derive(Serialize, Deserialize, Debug)]
pub struct AseTextureData {
    pub width: u32,
    pub height: u32,
    pub basename: String,
    pub frame: u32,
    pub x: u32,
    pub y: u32,
}

#[derive(Debug)]
pub struct AsepritePackerConfig<'a> {
    pub aseprite_file_names: &'a [&'a str],
    pub path: &'a Path,
    pub output_image_location: Option<&'a Path>,
    pub output_ron_location: Option<&'a Path>,
    pub trim: bool,
}

impl<'a> Default for AsepritePackerConfig<'a> {
    fn default() -> Self {
        AsepritePackerConfig {
            aseprite_file_names: &[],
            path: Path::new("."),
            output_image_location: None,
            output_ron_location: None,
            trim: false,
        }
    }
}

struct AseFile {
    path: PathBuf,
    name: String,
}

pub struct AsepritePacker {
    pub image: DynamicImage,
    pub packed_texture_data: HashMap<String, AseTextureData>,
}

impl AsepritePacker {
    pub fn new(config: AsepritePackerConfig) -> Self {
        let AsepritePackerConfig {
            aseprite_file_names,
            path,
            output_image_location,
            output_ron_location,
            trim,
        } = config;

        let texture_packer_config = TexturePackerConfig {
            max_width: std::u32::MAX,
            max_height: std::u32::MAX,
            allow_rotation: false,
            texture_outlines: false,
            border_padding: 0,
            texture_padding: 0,
            trim,
            ..Default::default()
        };

        let mut packer: TexturePacker<ImageBuffer<Rgba<u8>, Vec<u8>>, String> =
            TexturePacker::new_skyline(texture_packer_config);

        let mut packed_texture_data: HashMap<String, AseTextureData> = HashMap::default();

        let ase_files: Vec<AseFile> = if !aseprite_file_names.is_empty() {
            aseprite_file_names
                .iter()
                .map(|name| {
                    let resolved_name = format!("{}.aseprite", name);
                    AseFile {
                        path: path.clone().join(resolved_name.to_string()),
                        name: name.to_string(),
                    }
                })
                .collect()
        } else {
            println!("{}", path.display());
            let paths = fs::read_dir(path).unwrap();
            paths
                .map(|p| {
                    let path_buff = p.unwrap();
                    let name = path_buff
                        .path()
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string();
                    AseFile {
                        path: path_buff.path().as_path().to_owned(),
                        name,
                    }
                })
                .collect()
        };

        for file in ase_files {
            // let path = env::current_dir().unwrap();
            let ase_file = load_ase(file.path.as_path());
            match ase_file {
                Err(e) => panic!("{}", e),
                Ok(ase) => {
                    for frame_num in 0..ase.num_frames() {
                        ase.width();
                        let key: String = if ase.num_frames() > 1 {
                            format!("{}_{}", file.name.to_string(), frame_num)
                        } else {
                            file.name.to_string()
                        };
                        let _result = packer.pack_own(key.clone(), ase.frame(frame_num).image());
                        match _result {
                            Ok(_) => {}
                            Err(e) => panic!("Error packing file: {:?}", e),
                        }
                        let frame_data = packer.get_frame(&key).expect("Frame not found");
                        let source = frame_data.frame;
                        packed_texture_data.insert(
                            key.clone(),
                            AseTextureData {
                                width: source.w,
                                height: source.h,
                                x: source.x,
                                y: source.y,
                                basename: file.name.to_string(),
                                frame: frame_num,
                            },
                        );
                    }
                }
            }
        }

        let image = ImageExporter::export(&packer).unwrap();

        if let Some(output) = output_image_location {
            let mut file = std::fs::File::create(output).unwrap();
            image.write_to(&mut file, image::ImageFormat::Png).unwrap();
        }

        if let Some(output) = output_ron_location {
            let mut file = std::fs::File::create(output).unwrap();
            let str = to_string_pretty(&packed_texture_data, PrettyConfig::default()).unwrap();
            let _result = file.write_all(str.as_bytes());
            match _result {
                Ok(_) => {}
                Err(e) => panic!("{}", e),
            }
        }

        let ase_packer = AsepritePacker {
            packed_texture_data,
            image,
        };

        ase_packer
    }
}

fn load_ase(file_path: &Path) -> Result<AsepriteFile, AsepriteParseError> {
    let f = File::open(file_path).unwrap();
    AsepriteFile::read(&f)
}
