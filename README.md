# leaktk-jf

A utility/lib for taking json and writing it a directory tree to make
scanning certain things easier.

## WARNING - Very WIP

This project isn't "ready" yet. It still needs:

* Cleaning up
* Unittests
* To be turned into a lib
* Better security checks
* Better error handling


## Usage

```sh
rm -rf /tmp/jf-example
jf /tmp/jf-example < ./examples/file.json
```

Which should create something like this:

```
/tmp/jf-example/
├── key
└── nested
    ├── even
    │   ├── 0
    │   ├── 1
    │   │   └── with
    │   ├── 2
    │   ├── 3
    │   ├── 4
    │   └── 5
    └── keys
```
