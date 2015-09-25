use std::mem::replace;
use std::fs::{read_dir, ReadDir, DirEntry};
use std::path::{Path, PathBuf};

use filter::name_matches;
use {Error, ScanDir};

/// Iterator over pairs of (DirEntry, String) where latter is the file name
///
/// Iterator walks over files/directories in the depth-first order and doesn't
/// sort items any way. Only utf-8 decodable directory names are visited.
/// Same rules applied to both files and directories. If you want more
/// control, you may either filter files in the iterator itself or walk over
/// directory tree and use `ScanDir::read()` over files in each directory.
///
/// The iterator ensures that whole file name is utf-8 decodable,
/// so you may use `.to_str().unwrap()` on `file_name()`, `extension()`
/// and other methods of `entry.path()` that touch file name. But parent
/// path components may contain undecodable characters.
pub struct Walker<'a> {
    settings: &'a ScanDir,
    errors: &'a mut Vec<Error>,
    cur: Option<(ReadDir, PathBuf)>,
    stack: Vec<(ReadDir, PathBuf)>,
}

pub fn new<'x>(settings: &'x ScanDir, errors: &'x mut Vec<Error>,
    path: &'x Path)
    -> Walker<'x>
{
    let iter = read_dir(path).map_err(|e| {
        errors.push(Error::Io(e, path.to_path_buf()));
    }).ok().map(|i| (i, path.to_path_buf()));
    Walker {
        settings: settings,
        errors: errors,
        cur: iter,
        stack: Vec::new(),
    }
}

impl<'a> Iterator for Walker<'a> {
    type Item = (DirEntry, String);
    fn next(&mut self) -> Option<(DirEntry, String)> {
        loop {
            if let Some((ref mut iter, ref mut path)) = self.cur {
                match iter.next() {
                    Some(Ok(entry)) => {
                        let osname = entry.file_name();
                        if let Ok(name) = osname.into_string() {
                            if !name_matches(self.settings, &name) {
                                continue;
                            }
                            let typ = match entry.file_type() {
                                Ok(typ) => typ,
                                Err(e) => {
                                    self.errors.push(
                                        Error::Io(e, entry.path()));
                                    continue;
                                }
                            };
                            if typ.is_dir() {
                                let new_path = entry.path();
                                match read_dir(&new_path) {
                                    Ok(new_iter) => {
                                        let old_iter = replace(iter, new_iter);
                                        let old_path = replace(path, new_path);
                                        self.stack.push((old_iter, old_path));

                                    }
                                    Err(e) => {
                                        self.errors.push(
                                            Error::Io(e, entry.path()));
                                    }
                                }
                                if !self.settings.skip_dirs {
                                    return Some((entry, name));
                                }
                            } else {
                                if !self.settings.skip_files {
                                    return Some((entry, name));
                                }
                            }
                        } else {
                            self.errors.push(
                                Error::Decode(entry.path()));
                        }
                    }
                    Some(Err(e)) => {
                        self.errors.push(
                            Error::Io(e, path.to_path_buf()));
                    }
                    None => {
                        if let Some((new_iter, new_path)) = self.stack.pop() {
                            *iter = new_iter;
                            *path = new_path;
                        } else {
                            break;
                        }
                    }
                }
            } else {
                return None
            }
        }
        // We can only clean self.cur here because of borrowing rules
        self.cur = None;
        return None;
    }
}
