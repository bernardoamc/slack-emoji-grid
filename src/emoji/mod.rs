mod iterator;
use anyhow::Result;
use image::imageops;
use iterator::PartsIterator;
use reqwest::{multipart::Form, Client as ReqwestClient};
use std::path::PathBuf;

const GRID_SIZE: u32 = 640;
const GRID_ITEM_SIZE: u32 = 128;
const TOTAL_ROWS: u32 = GRID_SIZE / GRID_ITEM_SIZE;
const TOTAL_COLUMNS: u32 = GRID_SIZE / GRID_ITEM_SIZE;

pub struct Emoji {
    name: String,
    image_path: PathBuf,
    parts: Vec<EmojiPart>,
}

struct EmojiPart {
    image: String,
    name: String,
}

impl Emoji {
    pub fn new(name: &str, image_path: impl Into<PathBuf>) -> Self {
        Emoji {
            name: name.to_owned(),
            image_path: image_path.into(),
            parts: Vec::new(),
        }
    }

    fn resize(&self) -> Result<image::DynamicImage> {
        let image = image::open(&self.image_path)?;
        let resized_image =
            image.resize_exact(GRID_SIZE, GRID_SIZE, imageops::FilterType::Lanczos3);

        Ok(resized_image)
    }

    pub fn generate_parts(&mut self) -> Result<()> {
        let mut resized_image = self.resize()?;
        let parts_iterator = PartsIterator::new();

        for part in parts_iterator {
            let cropped_image = imageops::crop(
                &mut resized_image,
                part.x,
                part.y,
                GRID_ITEM_SIZE,
                GRID_ITEM_SIZE,
            );
            let part_name = format!("{}_{:0>2}_{:0>2}", self.name, part.column, part.row);
            let cropped_image_name = format!("{:0>2}_{:0>2}.png", part.column, part.row);
            cropped_image.to_image().save(&cropped_image_name)?;
            self.parts.push(EmojiPart {
                name: part_name,
                image: cropped_image_name,
            });
        }
        Ok(())
    }

    pub fn upload(&self, api_url: &str, slack_token: &str) -> Result<()> {
        for part in &self.parts {
            let form = Form::new()
                .text("token", slack_token.to_owned())
                .text("name", part.name.to_owned())
                .text("mode", "data")
                .file("image", &part.image)?;

            let response = ReqwestClient::new().post(api_url).multipart(form).send()?;

            if !response.status().is_success() {
                println!(
                    "{} couldn't be uploaded with name: {}",
                    part.image, part.name
                );
            }
        }
        Ok(())
    }

    pub fn print(&mut self) {
        for (i, part) in self.parts.iter().enumerate() {
            let index: u32 = (i as u32) + 1;
            print!(":{}:", part.name);
            if index % TOTAL_COLUMNS == 0 {
                println!("");
            }
        }
    }
}
