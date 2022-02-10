use crate::config::{Config, FolderConfig};
use std::vec::Vec;
use std::path::PathBuf;
use std::fs::ReadDir;
use std::io::Error;

pub fn sort( downloads_path: &PathBuf, file_paths: ReadDir, sorted_list: &mut Vec<Vec<String>>, config: Config) ->  u64{
    // count the files for the loading bar later
    let mut file_count: u64 = 0;
    let category_count = config.folderconfig.len();

    // iterate over the files in the folder 
    for file_path in file_paths {
        file_count += 1;
        let current_path = file_path.unwrap().file_name().into_string().unwrap();
        let kind = infer::get_from_path(downloads_path.join(&current_path));

        put_in_list(category_count, sorted_list, &config.folderconfig, current_path, kind);
    }
    file_count
}

fn put_in_list(category_count: usize, sorted_list: &mut Vec<Vec<String>>, folderconfig: &Vec<FolderConfig>, current_path: String, kind: Result<Option<infer::Type>, Error>) {

    let file_matchertype = match kind {
        Ok(res) => match res {
            Some(file_type) => file_type.matcher_type(),
            None => infer::MatcherType::CUSTOM,
        },
        Err(_) => infer::MatcherType::CUSTOM,
    };
    let file_matchertype = map_matchertype_to_string(file_matchertype);

    for index in 0..category_count {
        for keyword in &folderconfig[index].keywords {
            if current_path.contains(keyword) {
                println!("matched {} to {:?}", keyword, current_path);
                sorted_list[index].push(current_path.clone());
                return
            }
        }
        for matchertype in &folderconfig[index].matchertypes {
            if matchertype.eq(&file_matchertype) {
                println!("matched {} to {:?}", matchertype, file_matchertype);
                sorted_list[index].push(current_path.clone());
                return
            }
        }
        return
    }
}

fn map_matchertype_to_string(matchertype: infer::MatcherType) -> String {
    let mapping = match matchertype {
        infer::MatcherType::APP => "app",
        infer::MatcherType::ARCHIVE => "archive",
        infer::MatcherType::AUDIO => "audio",
        infer::MatcherType::BOOK => "book",
        infer::MatcherType::DOC => "document",
        infer::MatcherType::FONT => "font",
        infer::MatcherType::IMAGE => "image",
        infer::MatcherType::TEXT => "text",
        _ => "invalid",
    };
    mapping.to_string()
}
