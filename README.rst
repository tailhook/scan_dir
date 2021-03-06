========
Scan Dir
========

:Status: Beta
:Documentation: http://tailhook.github.com/scan_dir/

Simple interface to iterate over files or subdirs of a directory

Features:

1. Ensure that file names are decodable to utf-8
   (or error/warning is propagated)
2. Ignore hidden entries (by default)
3. Ignore common text editor and revision control backup files
4. Select only files or only directories
5. Simpler but detailed enough error handling
6. Recursive directory scanner

Here is the example:

.. code-block:: rust

    use scan_dir::ScanDir;

    ScanDir::dirs().read(".", |iter| {
        for (entry, name) in iter {
            println!("File {:?} has full path {:?}", name, entry.path());
        }
    }).unwrap()

Compare it to stdlib way:

.. code-block:: rust

    use std::fs::read_dir;
    for entry_res in read_dir(".").unwrap() {
        let entry = entry_res.unwrap();
        let file_name_buf = entry.file_name();
        let file_name = file_name_buf.to_str().unwrap();
        if !file_name.starts_with(".") &&
            entry.file_type().unwrap().is_dir()
        {
            println!("File {:?} has full path {:?}",
                file_name, entry.path());
        }
    }

Well, it looks almost fine until you want to turn unwrap's into correct
error reporting.


Upgrading
=========

The ``scan_dir`` 0.3 by default resolves symlink before checking if it's a file
or directory. In the ``scan_dir`` 0.1-0.2 symlinks where always included in the
list (i.e. they were skipped neither by ``skip_files`` nor by
``skip_dirs``).


License
=======

Licensed under either of

 * Apache License, Version 2.0, (./LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license (./LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.


Contribution
------------

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
