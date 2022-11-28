use std::{fs, path::Path};

use serde_json;

use crate::model::Task;

fn has_id(x: &Task) -> bool {
    x.id > 0
}

fn has_wh(x: &Task) -> bool {
    let result = x
        .label
        .iter()
        .any(|a| a.original_height.is_some() && a.original_width.is_some());
    if !result {
        println!(
            "warning image {} has no width or height, skipping.",
            &x.image
        );
    }
    result
}

fn is_collected(x: &Task, collected_images: &mut Vec<String>) -> bool {
    let result = collected_images.contains(&x.image);
    if result {
        println!("warning image {} was already collected. skipping", &x.image);
    } else {
        collected_images.push(x.image.clone());
    }
    result
}

pub fn deser_json(path: &Path) -> Result<Vec<Task>, std::io::Error> {
    let buf = fs::read_to_string(path)?;
    let deser = serde_json::from_str::<Vec<Task>>(&buf)?;
    let mut collected_images: Vec<String> = vec![];
    // TODO: turn into for loop for better readability
    let filtered: Vec<Task> = deser
        .into_iter()
        .filter(|x| {
            has_id(x) && has_wh(x) && is_collected(x, &mut collected_images)
        })
        .collect();
    println!("collected {} unique images.", collected_images.len());
    Ok(filtered)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn duplicate_image() {
        let input = Path::new("resources/duplicate_image.json");
        let data = crate::deser::deser_json(input).unwrap();
        assert_eq!(data.len(), 1)
    }

    #[test]
    fn no_width() {
        let input = Path::new("resources/no_width.json");
        let data = crate::deser::deser_json(input).unwrap();
        assert_eq!(data.len(), 0)
    }
}
