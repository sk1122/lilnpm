use std::io::BufReader;
use flate2::bufread::GzDecoder;
use reqwest::Client;
use tar::Archive;

pub fn download_package(module_name: &str, url: &str) {
    let res = reqwest::blocking::get(url).unwrap();

    let content = BufReader::new(res);
    let tarfile = GzDecoder::new(content);
    let mut archive = Archive::new(tarfile);

    archive.unpack(format!("./node_modules/{}", module_name)).unwrap();
}