use std::vec::Vec;
use std::path::PathBuf;
use std::fs::ReadDir;

pub fn sort( downloads_path: PathBuf, file_paths: ReadDir, sorted_list: &mut Vec<Vec<PathBuf>>) {
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
        infer::MatcherType::APP => sorted_list[0].push(full_path),
        infer::MatcherType::ARCHIVE => sorted_list[1].push(full_path),
        infer::MatcherType::AUDIO => sorted_list[2].push(full_path),
        infer::MatcherType::BOOK => sorted_list[3].push(full_path),
        infer::MatcherType::DOC => sorted_list[4].push(full_path),
        infer::MatcherType::FONT => sorted_list[5].push(full_path),
        infer::MatcherType::IMAGE => sorted_list[6].push(full_path),
        infer::MatcherType::TEXT => sorted_list[7].push(full_path),
        infer::MatcherType::VIDEO => sorted_list[8].push(full_path),
        _ => sorted_list[9].push(full_path)
    }
}
}

