# rvc
Rust Version Control

[Version control concepts and best practices](https://homes.cs.washington.edu/~mernst/advice/version-control.html)


index file contains:
<path-to-file>|<hash-of-file-content>

this is added to the index file by using the "add <file>" command.

should have a way of tracking the last version of each file.

when commiting, all it will compare the *tracked* files with each other, if there are any changes, it will make a blob of the file and store in the respective commit folder
the commit path should be divided into two directories, the first should contain the first 2 letters of the hashstring, and the second the rest.
to get the commit hash string we must join the commit number, commit message, and possibly the author.
inside the second folder should be the blob files correspondent to the commit
