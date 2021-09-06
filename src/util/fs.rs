use std::{cmp::Ordering, collections::HashMap, iter::FromIterator, path::{Path, PathBuf}};

use globwalk::{DirEntry, GlobError, GlobWalkerBuilder};
use lexical_sort::{natural_lexical_only_alnum_cmp};

use serde::Serialize;

trait LastNPath {
    fn last_n(&self, n: usize) -> String;
}

impl LastNPath for DirEntry {
    fn last_n(&self, n: usize) -> String {
        self.path().iter().rev().take(n).map(|x| x.to_string_lossy()).collect::<String>()
    }
}

impl LastNPath for std::fs::DirEntry {
    fn last_n(&self, n: usize) -> String {
        self.path().iter().rev().take(n).map(|x| x.to_string_lossy()).collect::<String>()
    }
}

fn sort_by_last_n<T: LastNPath>(n: usize, a: &T, b: &T) -> Ordering {
    natural_lexical_only_alnum_cmp(
        &a.last_n(n),
        &b.last_n(n)
    )
}

struct MangaDirs(HashMap<String, Vec<PathBuf>>);

impl FromIterator<(String, PathBuf)> for MangaDirs {
    fn from_iter<T: IntoIterator<Item = (String, PathBuf)>>(iter: T) -> Self {
        let h = iter.into_iter().fold(HashMap::new(), |mut acc, (str, path)| {
            acc.entry(str).or_insert_with(Vec::new).push(path);
            acc
        });

        MangaDirs(h)
    }
}

#[derive(Serialize)]
pub struct Manga {
    title: String,
    thumb: PathBuf,
}

/// Returns hash map of manga. Keys are names, values are paths to first ch
pub fn get_manga<'a>(manga_root: &'a Path) -> Result<Vec<Manga>, GlobError> {
    Ok(GlobWalkerBuilder::from_patterns(
        manga_root, 
        &["downloads/*/*", "local/*"])
        .file_type(globwalk::FileType::DIR)
        .sort_by(|a, b| sort_by_last_n(2, a, b))
        .build()?
        .filter_map(|dir_res|
            match dir_res {
                Ok(dir) => Some((dir.file_name().to_string_lossy().to_string(), dir.into_path())),
                Err(_) => None
            }
        ).collect::<MangaDirs>()
       .0.iter()
       .map(|(title, _)| {
           Manga{title: title.to_string(), thumb: PathBuf::new()}
       })
       .collect()
    )
}

fn construct_manga_ch_patterns(name: &str) -> Vec<String> {
    vec![
        format!("downloads/*/{}/*", name),
        format!("local/{}/*", name)
    ]
}


#[derive(Serialize)]
pub struct Chapter {
    title: String,
    thumb: PathBuf,
    path: PathBuf
}

pub fn get_chapters<'a>(manga_root: &Path, name: &'a str) -> Result<Vec<Chapter>, GlobError> {
    let out = GlobWalkerBuilder::from_patterns(
        manga_root,
        &construct_manga_ch_patterns(name))
        .sort_by(|a, b| sort_by_last_n(1, a, b))
        .build()?
        .filter_map(|dir_res| {
            let dir = dir_res.expect("Failed to get DirEntry");

            let name = dir.path().file_name()
                .and_then(|p| Some(p.to_string_lossy()));

            if let Some(n) = name {
                Some(
                    Chapter{
                        title: n.to_string(),
                        thumb: Path::new("/").to_path_buf(),
                        path: dir.into_path()
                    })
            } else {
                None
            }
        }).collect();
   
    Ok(out)
}
