use std::fs::{read_dir, ReadDir, DirEntry};
use std::path::Path;

use {Error, ScanDir};
use filter::matches;

/// Iterator over pairs of (DirEntry, String) where latter is the file name
///
/// The iterator ensures that whole file name is utf-8 decodable,
/// so you may use `.to_str().unwrap()` on `file_name()`, `extension()`
/// and other methods of `entry.path()` that touch file name. But some parts
/// of the path may be not utf-8 decodable anyway.
pub struct Iter<'a> {
    settings: &'a ScanDir,
    error: &'a mut Result<(), Error>,
    path: &'a Path,
    iter: Option<ReadDir>,
}

pub fn new<'x>(settings: &'x ScanDir, error: &'x mut Result<(), Error>,
    path: &'x Path)
    -> Iter<'x>
{
    let iter = read_dir(path).map_err(|e| {
        *error = Err(Error::Io(e, path.to_path_buf()));
    }).ok();
    Iter {
        settings: settings,
        error: error,
        path: path,
        iter: iter,
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (DirEntry, String);
    fn next(&mut self) -> Option<(DirEntry, String)> {
        loop {
            if let Some(ref mut iter) = self.iter {
                match iter.next() {
                    Some(Ok(entry)) => {
                        let osname = entry.file_name();
                        if let Ok(name) = osname.into_string() {
                            match matches(self.settings, &entry, &name) {
                                Ok(true) => return Some((entry, name)),
                                Ok(false) => {},
                                Err(e) => {
                                    error(self.error,
                                        Error::Io(e, entry.path()));
                                }
                            }
                        } else {
                            error(self.error,
                                Error::Decode(entry.path()));
                        }
                    }
                    Some(Err(e)) => {
                        error(self.error,
                            Error::Io(e, self.path.to_path_buf()));
                    }
                    None => return None,
                }
            } else {
                return None
            }
        }
    }
}

fn error(old: &mut Result<(), Error>, new: Error) {
    let overwrite = match (&old, &new) {
        (&&mut Ok(()), _) => true,
        // We overwrite Decode error by Io error because IO error usually
        // means that not all directory entries are read. This is critical
        // for most applications. But Decode error might be tolerated, i.e.
        // we just ignore configs which have invalid names.
        (&&mut Err(Error::Decode(_)), &Error::Io(_, _)) => true,
        _ => false,
    };
    if overwrite {
        *old = Err(new);
    }
}
