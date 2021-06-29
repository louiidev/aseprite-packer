use asefile::{AsepriteFile, AsepriteParseError};
use std::{fs, io::Write, path::PathBuf};
use image::{DynamicImage, ImageBuffer, Rgba};
use std::{collections::HashMap, fs::File, path::Path};
use texture_packer::{exporter::ImageExporter, TexturePacker, TexturePackerConfig };
use serde::{Serialize, Deserialize};
use ron::{ser::{PrettyConfig, to_string_pretty}, to_string};

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
    pub asesprite_file_names: &'a [&'a str],
    pub path: &'a Path,
    pub output_image_location: Option<&'a Path>,
    pub output_ron_location: Option<&'a Path>
}


struct AseFile {
    path: PathBuf,
    name: String
}


pub struct AsespritePacker {
    pub image: DynamicImage,
    pub packed_texture_data: HashMap<String, AseTextureData>,
}

impl AsespritePacker {
    pub fn new(config: AsepritePackerConfig) -> Self {

        let texture_packer_config = TexturePackerConfig {
            max_width: std::u32::MAX,
            max_height: std::u32::MAX,
            allow_rotation: false,
            texture_outlines: false,
            border_padding: 0,
            texture_padding: 0,
            ..Default::default()
        };
    
        let mut packer: TexturePacker<ImageBuffer<Rgba<u8>, Vec<u8>>, String> = TexturePacker::new_skyline(texture_packer_config);
    
        let mut packed_texture_data: HashMap<String, AseTextureData> = HashMap::default();

        
        let AsepritePackerConfig {
            asesprite_file_names,
            path,
            output_image_location,
            output_ron_location
        } = config;

        let ase_files: Vec<AseFile> = if !asesprite_file_names.is_empty() {
            asesprite_file_names.iter().map(|name| {
                let resolved_name = format!("{}.aseprite", name);
                AseFile {
                    path: path.clone().join(resolved_name.to_string()),
                    name: name.to_string()
                }
                
                }).collect()
            }
           else {
                println!("{}", path.display());
                let paths = fs::read_dir(path).unwrap();
                paths.map(|p| {
                    let path_buff = p.unwrap();
                    let name =  path_buff.path().file_stem().unwrap().to_str().unwrap().to_string();
                    AseFile {
                        path: path_buff.path().as_path().to_owned(),
                        name
                    }
                }).collect()
            };
        


        for file in ase_files {    
            // let path = env::current_dir().unwrap();
            let ase_file = load_ase(file.path.as_path());
            match ase_file {
                Err(e) => panic!(e),
                Ok(ase) => {
                    for frame in 0..ase.num_frames() {
                        ase.width();
                        let key: String = format!("{}_{}", file.name.to_string(), frame);
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
                                basename: file.name.to_string(),
                                frame,
                            },
                        );
                    }
                }
            }
        }

        let image = ImageExporter::export(&packer).unwrap();

        if let Some(output) = output_image_location {
            let mut file = std::fs::File::create(output).unwrap();
            image
                .write_to(&mut file, image::ImageFormat::Png)
                .unwrap();
            println!("Output texture stored in {:?}", file);
        }

        if let Some(output) = output_ron_location {
            let mut file = std::fs::File::create(output).unwrap();
            let str = to_string_pretty(&packed_texture_data, PrettyConfig::default()).unwrap();
            file.write_all(str.as_bytes());
            println!("Output texture stored in {:?}", file);
            
        }

        let ase_packer = AsespritePacker {
            packed_texture_data,
            image
        };


        ase_packer
    }
  
}


fn load_ase(file_path: &Path) -> Result<AsepriteFile, AsepriteParseError> {
    println!("{}", file_path.display());
    let f = File::open(file_path).unwrap();
    AsepriteFile::read(&f)
}
