# rvc
Simple Version Control System made in Rust

## What is it?
rvc is a simple Version Control System, inspired in git, that allows you to keep track of your changes locally (currently). You can configure where your commits are stored so you always know where to look for.

<!--
## How to install it?
??
-->

## Requirements
- [rust-lang](https://www.rust-lang.org/tools/install)

## How to use it?
### Init
To use `rvc` you first need to create your own project, then navigate to your project folder in your terminal of choice and initialize the rvc repository:

<!--
```bash
rvc init
```
-->
```bash
cargo run init
```

This will create the folder `.rvc`, which contains all the files and folders needed for `rvc` to manage the versions of your project.

### Config
You can configure your rvc with the following command:

<!--
```bash
rvc config <option> <value>
```
-->
```bash
cargo run config <option> <value>
```

Currently there is only one option available, which is the folder where your changes are stored:
- remote

### Add
After you've made some changes to your project you add the files you want to commit:

<!--
```bash
rvc add file1.rs 
```
-->
```bash
cargo run add file1.rs 
```

<!--
```bash
rvc add folder/
```
-->
```bash
cargo run add folder/
```

### Commit
When you are ready to make a commit, you can use the following command:

<!--
```bash
rvc commit "commit message"
```
-->
```bash
cargo run commit "commit message"
```

### Push
Finally, you can push your changes to your designated folder with this command:

<!--
```bash
rvc push
```
-->
```bash
cargo run push
```

### Cat File
If you want to see the contents of a gzip compressed file, for example, the files stored in your designated remote folder, you can use this command:

<!--
```bash
rvc cat-file file
```
-->
```bash
cargo run cat-file file
```

## Next steps
- Add the ability to `checkout`, making it posible to revert/go back to a different point in the project.
- Adding the ability to push to a remote using SSH.
- Add branches.
