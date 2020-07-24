use git2::{Error, Repository, RepositoryOpenFlags};
use once_cell::unsync::OnceCell;
use std::ffi::OsStr;
use std::path::Path;

pub fn get_repository(repo: &Path) -> Result<&Repository, Error> {
    static REPO: OnceCell<Repository> = OnceCell::new();
    REPO.get_or_try_init(|| {
        let repo = Repository::open_ext(
            repo.as_os_str(),
            RepositoryOpenFlags::empty(),
            vec![OsStr::new("")],
        );
        match repo {
            Ok(r) => Ok(r),
            Err(e) => Err(e),
        }
    })
}

fn main() {
    let path = Path::new(".");
    let res = get_repository(&path);
    println!("{}", &res.unwrap().path().display());
}
