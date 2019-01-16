extern crate futures;
extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate rusoto_credential;

use std::fs::File;
use std::io::Read;

use rusoto_core::Region;
use rusoto_core::credential::{ProfileProvider};

use rusoto_s3::{PutObjectRequest, S3Client, S3};

fn main() {
  match ProfileProvider::new() {
    Ok(result) => {
      println!("{:?}", result);
    },
    Err(err) => panic!("{:?}", err)
  };

  let region = Region::Custom {
      name: "ap-northeast-1".to_owned(),
      endpoint: "s3.ap-northeast-1.amazonaws.com".to_owned(),
  };
  let client = S3Client::new(region.clone());

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
