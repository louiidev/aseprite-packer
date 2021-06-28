mod packer;

fn main() {
    let atlas = packer::pack_ase_data(&["big", "small"], None);
    println!("{:?}", atlas.packed_texture_data.get("small_1"));
    let mut file = std::fs::File::create("src/skyline-packer-output.png").unwrap();
    atlas
        .image
        .write_to(&mut file, image::ImageFormat::Png)
        .unwrap();

    println!("Output texture stored in {:?}", file);
}
