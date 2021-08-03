use std::path::Path;
use std::collections::HashMap;

use globwalk::GlobWalkerBuilder;
use lexical_sort::natural_lexical_only_alnum_cmp;

/// Returns hash map of manga. Keys are names, values are paths to first ch
pub fn get_manga(manga_root: &Path) -> HashMap<String, Box<Path>> {
    let dirs: Vec<globwalk::DirEntry> = GlobWalkerBuilder::from_patterns(
    manga_root, 
    &["downloads/*/*/*", "local/*/*"])
    .file_type(globwalk::FileType::DIR)
    .sort_by(|a, b| 
        natural_lexical_only_alnum_cmp(
            &a.path().iter().rev().take(2).map(|x| x.to_string_lossy()).collect::<String>(),
            &b.path().iter().rev().take(2).map(|x| x.to_string_lossy()).collect::<String>())
    )
    .build().expect("AHH")
    .into_iter()
    .filter_map( |x| Some(x.expect("yes")))
    .collect();
}

pub fn get_chapters(name: &str) -> HashMap<String, Box<Path>> {

}