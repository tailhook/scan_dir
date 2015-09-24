//! Simple interface to iterate over files or subdirs of a directory
//!
//! Features:
//!
//! 1. Ensure that file names are decodable to utf-8
//!    (or error/warning is propagated)
//! 2. Ignore hidden entries (by default)
//! 3. Ignore common text editor and revision control backup files
//! 4. Select only files or only directories
//! 5. Simpler but detailed enough error handling
//!
//! For example you can read all subdirectories this way:
//!
//! ```rust
//! use scan_dir::ScanDir;
//!
//! ScanDir::dirs().read(".", |iter| {
//!     for (entry, name) in iter {
//!         println!("File {:?} has full path {:?}", name, entry.path());
//!     }
//! }).unwrap()
//! ```
//!
//! Compare it to stdlib way:
//!
//! ```rust
//! use std::fs::read_dir;
//! for entry_res in read_dir(".").unwrap() {
//!     let entry = entry_res.unwrap();
//!     let file_name_buf = entry.file_name();
//!     let file_name = file_name_buf.to_str().unwrap();
//!     if !file_name.starts_with(".") &&
//!         entry.file_type().unwrap().is_dir()
//!     {
//!         println!("File {:?} has full path {:?}",
//!             file_name, entry.path());
//!     }
//! }
//! ```
//! Well, it looks almost fine until you want to turn unwrap's into correct
//! error reporting.
//!
//! Here is a list of non-hidden rust files:
//!
//! ```rust
//! use scan_dir::ScanDir;
//!
//! let files: Vec<_> = ScanDir::files().read(".", |iter| {
//!     iter.filter(|&(_, ref name)| name.ends_with(".rs"))
//!         .map(|(entry, _)| entry.path())
//!         .collect()
//! }).unwrap();
//! ```
//!
//! And when you want to to return an from the closure, you need to
//! unify the errors somehow. For example:
//!
//! ```rust
//! use std::io;
//! use std::path::PathBuf;
//! use std::fs::File;
//! use scan_dir::ScanDir;
//!
//! #[derive(Debug)]
//! enum MyError {
//!     Scan(scan_dir::Error),
//!     File(io::Error, PathBuf),
//! }
//!
//! let result = ScanDir::files().read(".", |iter| {
//!     for (entry, name) in iter {
//!         // ensure file is accessible
//!         try!(File::open(entry.path())
//!             .map_err(|e| MyError::File(e, entry.path())));
//!     }
//!     Ok(())
//! }).map_err(MyError::Scan).and_then(|val| val);
//! println!("Error occured {:?}", result.err());
//! ```
//!
//! Let's see what's happen here:
//!
//! 1. The return value of `read()` is `Result<Result<(), io::Error>, MyError>`
//! 2. `map_err(MyError::Scan)` turns scan_dir error into our wrapper type
//! 3. `and_then(|val| val)` unifies the error, producing `Result<(), MyError>`
//!
//! Note in the last example you might convert error in the `and_then` clause,
//! but we want to know exact file name where error occured so we store it
//! in the error inside the lambda.
//!
#[macro_use] extern crate quick_error;

use std::io;
use std::path::{PathBuf, Path};

mod iter;

pub use iter::Iter;


quick_error! {
    /// Error type for scan dir
    ///
    /// It always contains the file name of the entry so display of an
    /// error is always an informative thing
    #[derive(Debug)]
    pub enum Error {
        /// I/O error occured when reading directory
        ///
        /// Second parameter contains path to the original directory
        Io(err: io::Error, path: PathBuf) {
            cause(err)
            display("error reading directory {:?}: {}", path, err)
            description("error reading directory")
        }
        /// Can't decode filename
        ///
        /// PathBuf contains path to the specific file which has bad filename
        Decode(path: PathBuf) {
            display("error decoding file name {:?}", path)
            description("error decoding file name")
        }
    }
}

/// Settings for directory walker
#[derive(Debug, Clone)]
pub struct ScanDir {
    skip_hidden: bool,
    skip_dirs: bool,
    skip_files: bool,
    skip_backup: bool,
}

impl ScanDir {
    /// Constructs a settings that iterates over all entries
    ///
    /// Just a starting point if you need complete control
    pub fn all() -> ScanDir {
        ScanDir {
            skip_hidden: false,
            skip_dirs: false,
            skip_files: false,
            skip_backup: false,
        }
    }
    /// Constructs a settings which only iterates over files (non-directories).
    ///
    /// The hidden and backup files are ignored by default
    pub fn files() -> ScanDir {
        ScanDir {
            skip_hidden: true,
            skip_dirs: true,
            skip_files: false,
            skip_backup: true,
        }
    }
    /// Constructs a settings which only iterates over directories
    ///
    /// The directories which match hidden and backup patterns are excluded
    pub fn dirs() -> ScanDir {
        ScanDir {
            skip_hidden: true,
            skip_dirs: false,
            skip_files: true,
            skip_backup: true,
        }
    }

    /// Skip hidden files
    ///
    /// Hidden files are the ones having the name starting with dot `.`
    /// (on all platforms)
    pub fn skip_hidden(&mut self, flag: bool) -> &mut ScanDir {
        self.skip_hidden = flag;
        self
    }

    /// Skip directory entries
    pub fn skip_dirs(&mut self, flag: bool) -> &mut ScanDir {
        self.skip_dirs = flag;
        self
    }

    /// Skip file (non-directory) entries
    pub fn skip_files(&mut self, flag: bool) -> &mut ScanDir {
        self.skip_files = flag;
        self
    }

    /// Skip backup files
    ///
    /// This is expected to skip entries that different editors and version
    /// control systems keep as backup files. We currently do not have precise
    /// control over this thing, so you may consider turn this off and filter
    /// entries yourself, if precise control is required.
    ///
    /// You may also just filter files by extension instead of using this.
    ///
    /// This is mostly useful if hidden files are also skipped.
    ///
    /// Currently we ignore the following patterns:
    ///
    /// * `*.bak`
    /// * `*~` -- vim/emacs/other backup files
    /// * `#*#` -- emacs auto save
    pub fn skip_backup(&mut self, flag: bool) -> &mut ScanDir {
        self.skip_files = flag;
        self
    }

    /// Calls a closure with an iterator over pairs of (entry, name)
    ///
    /// The method returns either error produced by scan_dir or Result
    /// returned by closure itself. The scan_dir::Error must be convertible
    /// to error produced by closure.
    ///
    /// Example:
    ///
    /// ```rust
    /// use scan_dir::ScanDir;
    ///
    /// ScanDir::files().read(".", |iter| {
    ///     for (entry, name) in iter {
    ///         println!("File {:?} has full path {:?}",
    ///             name, entry.path());
    ///     }
    /// }).unwrap()
    /// ```
    pub fn read<P:AsRef<Path>, R:Sized, F>(&self, path: P, f: F)
        -> Result<R, Error>
        where F: FnOnce(Iter) -> R
    {
        let mut dir_res = Ok(());
        let user_res = f(iter::new(self, &mut dir_res, path.as_ref()));
        dir_res.and(Ok(user_res))
    }
}
