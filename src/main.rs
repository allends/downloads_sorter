mod config;
mod sorter;
use sorter::sort;

mod file_move;
use file_move::move_to_archive;

mod initializer;
use initializer::setup;

extern crate fs_extra;
extern crate dirs;

extern crate infer;

fn main() {

    // SECTION ONE: SET UP OS SPECIFIC STUFF
    let (downloads_path, archive_base_directory, mut sorted_list, file_paths, categories, config) = setup();


    // SECTION THREE: SORT THE FILES IN THE DOWNLOADS FOLDER
    let file_count = sort(&downloads_path, file_paths, &mut sorted_list, config);


    // SECTION FOUR: MOVE THE FILES INTO THEIR FOLDERS
    move_to_archive(&downloads_path, archive_base_directory, sorted_list, categories, file_count);
}
