# passgen

A port of [@bradtraversy](https://github.com/bradtraversy)'s [passgen](https://github.com/bradtraversy/passgen) project in Rust.

## Usage

Build project

```
cargo build --release
```

Run file

```
./passgen (options)
```

## Options

| Short | Long              | Description                     |
| ----- | ----------------- | ------------------------------- |
| -l    | --length <number> | length of password [default: 8] |
| -s    | --save            | save password to passwords.txt  |
|       | --no-numbers      | remove numbers                  |
|       | --no-symbols      | remove symbols                  |
| -h    | --help            | display usage information       |
| -V    | --version         | Show the version                |
