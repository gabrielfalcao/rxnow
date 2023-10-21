# rxnow

![docs/nightingale.svg](docs/nightingale.svg)


Experimental drop-in replacement for GNU/Sed

## Install


```bash
cargo install rxnow
```


## Examples


```bash
curl -qs https://raw.githubusercontent.com/gabrielfalcao/rxnow/main/README.md | rxnow ".*(curl.*?[.]md).*" --replace '$1'
```


```bash
$ rxnow --help

aims at sorta-kinda drop-in replace GNU/Sed et al.

Usage: rxnow [OPTIONS] <EXPRESSION> [FILENAMES]...

Arguments:
  <EXPRESSION>    the regex pattern
  [FILENAMES]...  list of files wherein search shall happen. Defaults to stdin if none is provided

Options:
  -i, --ignore-case                  whether to ignore case
  -I, --ignore-spaces                whether to ignore spaces
  -r, --replace <REPLACEMENT>        replace (ft. group matching)
  -n, --no-newline
  -f, --show-filename
  -a, --achromatic                   disable colored output
  -c, --count                        counts occurrences of regex
  -o, --omit-empty                   omits empty lines
  -t, --trim                         strip space characters at both ends of each line
  -d, --delete                       deletes portions of input-data matching the given expression
  -l, --files-with-matches
  -C, --context <NUM>
  -g, --group-color <GROUP_COLOR>    [default: 220]
  -m, --match-color <MATCH_COLOR>    [default: 154]
  -s, --source-color <SOURCE_COLOR>  [default: 202]
  -h, --help                         Print help
  -V, --version                      Print version
```
