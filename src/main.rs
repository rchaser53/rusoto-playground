extern crate walkdir;
extern crate zip;

extern crate futures;
extern crate rusoto_core;
extern crate rusoto_credential;
extern crate rusoto_dynamodb;

use std::fs::File;
use std::io::{Read, Seek, Write};
use std::path::Path;

use walkdir::{DirEntry, WalkDir};
use zip::result::ZipError;
use zip::write::FileOptions;

use rusoto_core::credential::ProfileProvider;
use rusoto_core::Region;
use rusoto_s3::{PutObjectRequest, S3Client, S3};

fn main() {
    match ProfileProvider::new() {
        Ok(result) => {
            println!("{:?}", result);
        }
        Err(err) => panic!("{:?}", err),
    };

    let region = Region::Custom {
        name: "ap-northeast-1".to_owned(),
        endpoint: "s3.ap-northeast-1.amazonaws.com".to_owned(),
    };
    let client = S3Client::new(region.clone());
    create_zip("hoge", "result.zip");

    let mut f = File::open("README.md").unwrap();
    let mut contents: Vec<u8> = Vec::new();
    match f.read_to_end(&mut contents) {
        Err(why) => panic!("Error opening file to send to S3: {}", why),
        Ok(_) => {
            let req = PutObjectRequest {
                bucket: "rchaser53-testbacket".to_owned(),
                key: "nyan".to_owned(),
                body: Some(contents.into()),
                ..Default::default()
            };
            let result = client.put_object(req).sync().expect("Couldn't PUT object");
            println!("{:#?}", result);
        }
    }
}

fn create_zip(src_dir: &str, dst_file: &str) {
    let path = Path::new(dst_file);
    let file = File::create(&path).unwrap();

    let walkdir = WalkDir::new(src_dir.to_string());
    let it = walkdir.into_iter();

    match zip_dir(&mut it.filter_map(|e| e.ok()), src_dir, file) {
        Ok(_) => {}
        Err(err) => {
            panic!("{:?}", err);
        }
    };
}

fn zip_dir<T>(
    it: &mut Iterator<Item = DirEntry>,
    prefix: &str,
    writer: T,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path
            .strip_prefix(Path::new(prefix))
            .unwrap()
            .to_str()
            .unwrap();

        if path.is_file() {
            println!("adding {:?} as {:?} ...", path, name);
            zip.start_file(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        }
    }
    zip.finish()?;
    Result::Ok(())
}
