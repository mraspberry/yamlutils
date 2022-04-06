use serde_json;
use serde_yaml;
use serde_transcode::transcode;

use std::io;

fn main() {
    let mut deserializer = serde_yaml::Deserializer::new(&io::stdin());
    let mut serializer = serde_json::Serializer::pretty(&io::stdout());
    transcode(&deserializer, &serializer).unwrap();
}
