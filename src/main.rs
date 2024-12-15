use image_hasher::{HasherConfig};
use std::fs;

fn main() {
    let hasher = HasherConfig::new().to_hasher();
    let current_dir = std::env::current_dir().unwrap();

    for entry in fs::read_dir(current_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if let Some(extension) = path.extension() {
            if extension == "png" || extension == "jpg" || extension == "jpeg" || extension == "tiff" {
                match image::open(&path) {
                    Ok(image) => {
                        let hash = hasher.hash_image(&image);
                        println!("Hash de {} : {}", path.display(), hash.to_base64());
                    }
                    Err(e) => {
                        eprintln!("Erreur pendant l'ouverture de l'image {}: {:?}", path.display(), e);
                    }
                }
            }
        }
    }
}
