# renparkn
Recursively rename files in provided directory by adding the parent directory name while keeping the numbering (only first number in file name is kept).

## Usage
```
renparkn [OPTIONS] <DIR_PATH>
```

#### Arguments
```
<DIR_PATH>  Path to directory with files to be renamed
```

#### Options
```
-a, --num-after <STRING>  Extract numbering after provided string (case sensitive)
-n, --dry-run             Show rename proposal but do not apply
-h, --help                Print help
-V, --version             Print version
```
