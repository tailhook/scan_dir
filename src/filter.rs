use std::io;
use std::fs::{DirEntry, metadata};

use {ScanDir};

/// Checks for rules of the entry
///
/// Keep in sync with documentation of ScanDir class itself
pub fn matches(s: &ScanDir, entry: &DirEntry, name: &String)
    -> Result<bool, io::Error>
{
    if !name_matches(s, name) {
        return Ok(false);
    }
    if s.skip_dirs || s.skip_files {
        let mut typ = try!(entry.file_type());
        if typ.is_symlink() {
            if s.skip_symlinks {
                return Ok(false);
            } else {
                typ = try!(metadata(entry.path())).file_type();
            }
        }
        if s.skip_dirs && typ.is_dir() {
            return Ok(false);
        }
        if s.skip_files && typ.is_file() {
            return Ok(false);
        }
    }
    return Ok(true);
}

pub fn name_matches(s: &ScanDir, name: &String) -> bool {
    if s.skip_hidden && name.starts_with(".") {
        return false;
    }
    if s.skip_backup {
        if name.ends_with("~") {
            return false;
        }
        if name.ends_with(".bak") {
            return false;
        }
        if name.starts_with("#") && name.ends_with("#") {
            return false;
        }
    }
    return true;
}
