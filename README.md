# mdbook-readme-summary

## Usecase

When you are using mdbook and you have the following file structure:

.
└── src/
    ├── subdirectory1/
    │   ├── subdirectory1.1/
    │   │   ├── file1.md
    │   │   ├── ...
    │   │   └── README.md
    │   ├── file1.md
    │   ├── file2.md
    │   ├── ...
    │   └── README.md
    ├── subdirectory2/
    │   ├── file1.md
    │   ├── file2.md
    │   ├── ...
    │   └── README.md
    ├── ...
    └── README.md

and you want a table of content within the readme in each subdirectory showing the content of that directory.

## Usage

In your README.md files, add `{{TOC}}` in a desired location.

In your book.toml add 
```
[preprocessor.readme-summary]
```

There are two configuration that you can use:

```
[preprocessor.readme-summary]
enable-draft = true
enable-log = true
```

This preprocessor automatically ignore files and directories that names containing the keyword "(draft)" by default.Setting `enable-draft` to true will disable that ignore.

Setting `enable-log` to true will show some logging information to debugging.

