use ico::{IconDir, IconDirEntry, IconImage};
const ICON_SOURCE_FILE_PATH: &str = "./ass/icon/icon256.png";
const WIN_ICO_PATH: &str = "./ass/icon/icon.ico";

fn main() {
    //Windows only
    //Process icon for exe
    if cfg!(target_os = "windows") {
        println!("cargo:rerun-if-changed={}", ICON_SOURCE_FILE_PATH);
        //Process icon data
        let img = image::open(ICON_SOURCE_FILE_PATH)
            .expect(format!("Cannot open: {}", ICON_SOURCE_FILE_PATH).as_str());

        let icon_sizes = [16, 32, 48, 64, 128, 256];

        let mut icon_dir: IconDir = IconDir::new(ico::ResourceType::Icon);

        for &size in &icon_sizes {
            let resized_img_rgba = img
                .resize_exact(size, size, image::imageops::Lanczos3)
                .to_rgba8();
            let icon_img: IconImage =
                IconImage::from_rgba_data(size, size, resized_img_rgba.into_raw());
            icon_dir.add_entry(IconDirEntry::encode(&icon_img).unwrap());
        }

        let path = std::path::Path::new(WIN_ICO_PATH);
        let file = std::fs::File::create(&path).unwrap();
        icon_dir.write(file).unwrap();

        //Embed to exe
        let mut res = winres::WindowsResource::new();
        res.set_icon(WIN_ICO_PATH);
        res.compile().unwrap();
    }

    // macOS: notify to use cargo-bundle
    if cfg!(target_os = "macos") {
        println!("macOS: use cargo-bundle to embed icon");
    }
}
