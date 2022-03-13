use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::path::PathBuf;
use tar::{Archive, Builder};
use tokio::fs::{remove_file, File};

pub async fn force_create(path: PathBuf) -> File {
    if path.exists() {
        remove_file(path.to_owned()).await.unwrap();
    }
    File::create(path.to_owned()).await.unwrap()
}

pub async fn un_gzip(path: PathBuf, to: PathBuf) {
    let file = File::open(path).await.unwrap();
    let dec = GzDecoder::new(file.into_std().await);
    let mut arch = Archive::new(dec);
    arch.unpack(to).unwrap()
}

pub async fn gzip(from: PathBuf, to: PathBuf, dname: String) {
    let out = File::create(to).await.unwrap();
    let enc = GzEncoder::new(out.into_std().await, Compression::default());
    let mut builder = Builder::new(enc);
    builder.append_dir_all(from, dname).unwrap();
    builder.finish().unwrap()
}
