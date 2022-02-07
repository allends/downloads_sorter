use std::fs;
use std::env;
use std::vec::Vec;
use std::path::PathBuf;

extern crate fs_extra;
extern crate dirs;
use fs_extra::dir::*;
use fs_extra::move_items;

extern crate infer;

fn main() {

    // SECTION ONE: SET UP OS SPECIFIC STUFF
    println!("{}", env::consts::OS);
    let downloads_pathbuf = dirs::download_dir().unwrap();
    let downloads_path = downloads_pathbuf.as_path();
    let file_paths = fs::read_dir(downloads_path).unwrap();

    // TODO: MAKE THIS CHANGE BASED ON OS
    let archive_base_buf = dirs::home_dir().unwrap();
    let archive_base_directory = archive_base_buf.as_path().join("Archives");

    // SECTION TWO: MAKE THE FOLDERS FOR EACH CATEGORY
    let categories = ["App", "Archive", "Audio", "Book", "Doc", "Font", "Image", "Text", "Video", "Other"];

    let mut categories_list: Vec<Vec<PathBuf>> = Vec::new();
    for category in categories {
        categories_list.push(Vec::new());
        let category_path = archive_base_directory.join(category);
        let create_check_result = create_all(&category_path, false);
        match create_check_result {
            Ok(_) => println!("Category Folder Set Up!"),
            Err(e) => println!("Oh no! {}", e),
        }
        assert!(category_path.exists());
    }

    // SECTION THREE: SORT THE FILES IN THE DOWNLOADS FOLDER
    for file_path in file_paths {
        let current_path = file_path.unwrap().file_name().into_string().unwrap();
        let kind = infer::get_from_path(downloads_path.join(&current_path));

        let result = match kind {
            Ok(res) => match res {
                Some(file_type) => file_type.matcher_type(),
                None => infer::MatcherType::CUSTOM,
            },
            Err(_) => infer::MatcherType::CUSTOM,
        };
        let full_path = downloads_path.join(current_path);
        match result {
            infer::MatcherType::APP => categories_list[0].push(full_path),
            infer::MatcherType::ARCHIVE => categories_list[1].push(full_path),
            infer::MatcherType::AUDIO => categories_list[2].push(full_path),
            infer::MatcherType::BOOK => categories_list[3].push(full_path),
            infer::MatcherType::DOC => categories_list[4].push(full_path),
            infer::MatcherType::FONT => categories_list[5].push(full_path),
            infer::MatcherType::IMAGE => categories_list[6].push(full_path),
            infer::MatcherType::TEXT => categories_list[7].push(full_path),
            infer::MatcherType::VIDEO => categories_list[8].push(full_path),
            _ => categories_list[9].push(full_path)
        }
    }


    // SECTION FOUR: MOVE THE FILES INTO THEIR FOLDERS
    let options = fs_extra::dir::CopyOptions::new();
    for (index, list) in categories_list.iter().enumerate() {
        let destination_folder = archive_base_directory.join(categories[index]);
        let move_result = move_items(&list, &destination_folder, &options);
        match move_result {
            Ok(_) => println!("Moved all of the {} files", categories[index]),
            Err(e) => println!("Something went terribly wrong :'( INFO: {}", e),
        }
        println!();
    }

}
