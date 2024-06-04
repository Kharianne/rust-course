use image::ImageFormat;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::io::Cursor;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Text(String),
    Image(Vec<u8>),
    File(String, Vec<u8>),
}

impl MessageType {
    pub fn serialize_message(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn deserialize_message(data: &[u8]) -> MessageType {
        serde_json::from_slice(data).unwrap()
    }
}

impl FromStr for MessageType {
    type Err = Box<dyn Error>;

    fn from_str(message: &str) -> Result<Self, Self::Err> {
        let (message_type, path) = prepare_message(message)?;
        println!("message_type: {message_type}, path: {path}");
        match message_type.as_str() {
            "text" => Ok(MessageType::Text(path)),
            "image" => {
                let image_data = fs::read(&path)?;
                let image_data = convert_to_png(image_data)?;
                Ok(MessageType::Image(image_data))
            }
            "file" => {
                let file_data = fs::read(&path)?;
                Ok(MessageType::File(path, file_data))
            }
            _ => Err(
                "Invalid message type - write your text or specify .image/.file to send files."
                    .into(),
            ),
        }
    }
}

fn prepare_message(message: &str) -> Result<(String, String), Box<dyn Error>> {
    let parts: Vec<&str> = message.splitn(2, char::is_whitespace).collect();
    print!("{:?}", parts);
    match parts[0] {
        ".image" => Ok(("image".to_string(), parts[1].to_string())),
        ".file" => Ok(("file".to_string(), parts[1].to_string())),
        _ => Ok(("text".to_string(), message.to_string())),
    }
}
fn convert_to_png(img_bytes: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
    let img = image::load_from_memory(&img_bytes)?;
    let format = image::guess_format(&img_bytes)?;
    if format == ImageFormat::Png {
        Ok(img_bytes)
    } else {
        let mut png_bytes = Vec::new();
        img.write_to(&mut Cursor::new(&mut png_bytes), ImageFormat::Png)?;
        Ok(png_bytes)
    }
}
