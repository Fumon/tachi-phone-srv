use lexical_sort::natural_lexical_only_alnum_cmp;

pub trait LastNPath {
    fn last_n(&self, n: usize) -> String;
}

impl LastNPath for globwalk::DirEntry {
    fn last_n(&self, n: usize) -> String {
        self.path().iter().rev().take(n).map(|x| x.to_string_lossy()).collect::<String>()
    }
}

impl LastNPath for std::fs::DirEntry {
    fn last_n(&self, n: usize) -> String {
        self.path().iter().rev().take(n).map(|x| x.to_string_lossy()).collect::<String>()
    }
}

pub fn sort_lex_by_last_n<T: LastNPath>(n: usize, a: &T, b: &T) -> std::cmp::Ordering {
    natural_lexical_only_alnum_cmp(
        &a.last_n(n),
        &b.last_n(n)
    )
}
