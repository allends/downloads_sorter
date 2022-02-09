use std::vec::Vec;
use std::path::PathBuf;
use std::fs::ReadDir;

pub fn sort( downloads_path: &PathBuf, file_paths: ReadDir, sorted_list: &mut Vec<Vec<String>>) ->  u64{
    let mut file_count: u64 = 0;
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
        let file_name = current_path;
        match result {
            infer::MatcherType::APP => sorted_list[0].push(file_name),
            infer::MatcherType::ARCHIVE => sorted_list[1].push(file_name),
            infer::MatcherType::AUDIO => sorted_list[2].push(file_name),
            infer::MatcherType::BOOK => sorted_list[3].push(file_name),
            infer::MatcherType::DOC => sorted_list[4].push(file_name),
            infer::MatcherType::FONT => sorted_list[5].push(file_name),
            infer::MatcherType::IMAGE => sorted_list[6].push(file_name),
            infer::MatcherType::TEXT => sorted_list[7].push(file_name),
            infer::MatcherType::VIDEO => sorted_list[8].push(file_name),
            _ => sorted_list[9].push(file_name)
        }
        file_count += 1;
    }
    file_count
}

