use std::io;
use std::fs::{read_dir, ReadDir, DirEntry};
use std::path::Path;

use {Error, ScanDir};

/// Iterator over pairs of (DirEntry, String) where latter is file stem
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
                            // We don't use path().extension() here because
                            // that's joins path = does allocation of the
                            // path. We avoid it as much as possible
                            match matches(self.settings, &entry, &name) {
                                Ok(true) => return Some((entry, name)),
                                Ok(false) => {},
                                Err(e) => {
                                    if self.error.is_ok() {
                                        // only if there was no error yet
                                        *self.error = Err(
                                            Error::Io(e, entry.path()));
                                    }
                                }
                            }
                        } else {
                            if self.error.is_ok() {
                                // only if there was no error yet
                                *self.error = Err(Error::Decode(entry.path()));
                            }
                        }
                    }
                    Some(Err(e)) => {
                        if self.error.is_ok() {
                            // only if there was no error yet
                            *self.error = Err(Error::Io(e,
                                self.path.to_path_buf()));
                        }
                    }
                    None => return None,
                }
            } else {
                return None
            }
        }
    }
}

/// Checks for rules of the entry
///
/// Keep in sync with documentation of ScanDir class itself
fn matches(s: &ScanDir, entry: &DirEntry, name: &String)
    -> Result<bool, io::Error>
{
    if s.skip_hidden && name.starts_with(".") {
        return Ok(false);
    }
    if s.skip_backup {
        if name.ends_with("~") {
            return Ok(false);
        }
        if name.ends_with(".bak") {
            return Ok(false);
        }
        if name.starts_with("#") && name.ends_with("#") {
            return Ok(false);
        }
    }
    if s.skip_dirs || s.skip_files {
        let typ = try!(entry.file_type());
        if s.skip_dirs && typ.is_dir() {
            return Ok(false);
        }
        if s.skip_files && typ.is_file() {
            return Ok(false);
        }
    }
    return Ok(true);
}
