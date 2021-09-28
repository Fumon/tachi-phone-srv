use std::{borrow::Cow, path::{Path, PathBuf}};

use globwalk::GlobWalkerBuilder;

use crate::util::dirs::sort_lex_by_last_n;

/// Contains the path to a file
#[derive(Debug)]
pub struct MangaImage {
    path: PathBuf,
    chapter: usize // Might be index
}

impl MangaImage {
    pub fn get_pagenum(&self) -> Result<usize, String> {
        match self.path.file_stem().unwrap()
            .to_string_lossy()
            .trim_matches(|c: char| !c.is_numeric())
            .parse() {
                Ok(num) => Ok(num),
                Err(e) => Err("bad int".to_string())
            }
    }

    pub fn lex_sorted_name(&self) -> Result<String, String> {
        let pn = self.get_pagenum()?;
        let ext = match self.path.extension() {
            Some(e) => e.to_string_lossy(),
            None => return Err("No extension".to_string())
        };
        
        Ok(format!("{:04}{:03}.{}",
                    self.chapter,
                    pn,
                    ext
                ))
    }
}

pub struct ImagePack<T: Iterator<Item=MangaImage>> {
    filewalker: T
}

impl<T> ImagePack<T> 
    where T: Iterator<Item=MangaImage>
{
    fn from_iter(iter: T) -> Self {
        ImagePack { filewalker: iter }
    }
}

impl<T> Iterator for ImagePack<T> 
    where T: Iterator<Item=MangaImage>
{
    type Item = MangaImage;

    fn next(&mut self) -> Option<Self::Item> {
        self.filewalker.next()
    }
}

enum ChapterRange {
    All,
    Range(std::ops::Range<usize>),
    Count(usize)
}

pub struct SeriesGetter<'a> {
    name: Cow<'a, str>,
    chapter_range: ChapterRange
}

impl<'b> SeriesGetter<'b> {
    
    // Constructors
    pub fn from_name<'a>(name: &'a str) -> SeriesGetter<'a> {
        SeriesGetter {
            name: Cow::Borrowed(name),
            chapter_range: ChapterRange::All
        }
    }

    pub fn all_chapters<'a>(&'a mut self) -> &'a mut Self {
        self.chapter_range = ChapterRange::All;
        self
    }

    // Ordinal range. Will make best effort to recover chapter numbers and return those chapters within given range.
    pub fn chapter_range(& mut self, range: std::ops::Range<usize>) -> & mut Self {
        self.chapter_range = ChapterRange::Range(range);
        self
    }

    pub fn chapter_count(& mut self, count: usize) -> & mut Self {
        self.chapter_range = ChapterRange::Count(count);
        self
    }

    pub fn get_chapters(&self) {
        unimplemented!("Not sure if needed yet")
    }

    fn construct_manga_ch_patterns(&self) -> Vec<String> {
        vec![
            format!("downloads/*/{}/*", self.name),
            format!("local/{}/*", self.name)
        ]
    }
    
    // Returns an iterator of images with sorted, monotonic names
    pub fn get_image_pack(&self, root: &Path) -> ImagePack<impl Iterator<Item=MangaImage>> {
        let chapterwalker = GlobWalkerBuilder::from_patterns(root, &self.construct_manga_ch_patterns())
            .sort_by(|a, b| sort_lex_by_last_n(1, a, b))
            .build().unwrap();
        
        let chap_img_walker = chapterwalker.enumerate().flat_map(|(i, dirres)| {
            let dir = dirres.expect("Couldn't unwrap direntry");

            std::fs::read_dir(dir.path()).unwrap().map(move |a| MangaImage{path: a.unwrap().path(), chapter: i})
        });

        ImagePack::from_iter(chap_img_walker)
    }
}