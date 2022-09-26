# ipyprev

A CLI tool for quick ipynb preview.

![](images/image-0.png)

### Installation

```shell
$ cargo install --git https://github.com/hanjinliu/ipyprev
```

And it's ready for preview

```shell
$ ipyprev path/to/notebook.ipynb
```

### Usage

```shell
$ ipyprev --help
```

```
USAGE:
    ipyprev.exe [FLAGS] <file>

FLAGS:
    -h, --help         Prints help information
        --no-output    Do not show cell output
        --plain        Print without syntax highlighting
    -V, --version      Prints version information

ARGS:
    <file>    The ipynb file to preview
```