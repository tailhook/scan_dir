var searchIndex = {};
searchIndex["scan_dir"] = {"doc":"Simple interface to iterate over files or subdirs of a directory","items":[[3,"Iter","scan_dir","Iterator over pairs of (DirEntry, String) where latter is the file name",null,null],[3,"Walker","","Iterator over pairs of (DirEntry, String) where latter is the file name",null,null],[3,"ScanDir","","Settings for directory walker",null,null],[4,"Error","","Error type for scan dir",null,null],[13,"Io","","I/O error occured when reading directory",0,null],[13,"Decode","","Can&#39;t decode filename",0,null],[11,"next","","",1,{"inputs":[{"name":"iter"}],"output":{"name":"option"}}],[11,"exit_current_dir","","Premature exit from directory that we are currently scanning",2,{"inputs":[{"name":"walker"}],"output":null}],[11,"next","","",2,{"inputs":[{"name":"walker"}],"output":{"name":"option"}}],[11,"fmt","","",0,{"inputs":[{"name":"error"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",0,{"inputs":[{"name":"error"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"description","","",0,{"inputs":[{"name":"error"}],"output":{"name":"str"}}],[11,"cause","","",0,{"inputs":[{"name":"error"}],"output":{"name":"option"}}],[11,"clone","","",3,{"inputs":[{"name":"scandir"}],"output":{"name":"scandir"}}],[11,"fmt","","",3,{"inputs":[{"name":"scandir"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"all","","Constructs a settings that iterates over all entries",3,{"inputs":[],"output":{"name":"scandir"}}],[11,"files","","Constructs a settings which only iterates over files (non-directories).",3,{"inputs":[],"output":{"name":"scandir"}}],[11,"dirs","","Constructs a settings which only iterates over directories",3,{"inputs":[],"output":{"name":"scandir"}}],[11,"skip_hidden","","Skip hidden files",3,{"inputs":[{"name":"scandir"},{"name":"bool"}],"output":{"name":"scandir"}}],[11,"skip_dirs","","Skip directory entries",3,{"inputs":[{"name":"scandir"},{"name":"bool"}],"output":{"name":"scandir"}}],[11,"skip_files","","Skip file (non-directory) entries",3,{"inputs":[{"name":"scandir"},{"name":"bool"}],"output":{"name":"scandir"}}],[11,"skip_symlinks","","Skip symlinks",3,{"inputs":[{"name":"scandir"},{"name":"bool"}],"output":{"name":"scandir"}}],[11,"skip_backup","","Skip backup files",3,{"inputs":[{"name":"scandir"},{"name":"bool"}],"output":{"name":"scandir"}}],[11,"read","","Calls a closure with an iterator over pairs of (entry, name)",3,{"inputs":[{"name":"scandir"},{"name":"p"},{"name":"f"}],"output":{"name":"result"}}],[11,"walk","","Calls a closure with recursive walker over the directory",3,{"inputs":[{"name":"scandir"},{"name":"p"},{"name":"f"}],"output":{"name":"result"}}]],"paths":[[4,"Error"],[3,"Iter"],[3,"Walker"],[3,"ScanDir"]]};
searchIndex["quick_error"] = {"doc":"A macro which makes errors easy to write","items":[[14,"quick_error!","quick_error","Main macro that does all the work",null,null]],"paths":[]};
initSearch(searchIndex);