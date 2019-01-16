extern crate futures;
extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate rusoto_credential;

use std::default::Default;
use std::collections::HashMap;

use rusoto_core::Region;
// use rusoto_core::credential::EnvironmentProvider;
use rusoto_core::credential::ProfileProvider;

use rusoto_dynamodb::{AttributeValue, BatchGetItemInput, DynamoDb, DynamoDbClient, KeysAndAttributes};
// use futures::Future;

fn main() {
  // let _ = EnvironmentProvider::default().credentials().wait();
  match ProfileProvider::new() {
    Ok(result) => {
      println!("{:?}", result);
    },
    Err(err) => panic!("{:?}", err)
  };

  let client = DynamoDbClient::new(Region::ApNortheast1);

  let mut request_items: HashMap<String, KeysAndAttributes> = HashMap::new();
  let mut attribute_value: HashMap<String, AttributeValue> = HashMap::new();
  attribute_value.insert(
    String::from("id"),
    AttributeValue{
      s: Some(String::from("test")),
      ..Default::default()
    });

  request_items.insert(String::from("test_dynamo"), KeysAndAttributes{
    keys: vec![ attribute_value ],
    ..Default::default()
  });

  match client.batch_get_item(BatchGetItemInput {  
    request_items,
    return_consumed_capacity: None
  }).sync() {
    Ok(output) => {
      println!("{:?}", output);
    },
    Err(err) => {
      panic!("{:?}", err);
    }
  };
}
