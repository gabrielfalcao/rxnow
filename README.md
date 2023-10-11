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
  -r, --replace <REPLACEMENT>      replace (ft. group matching)
  -n, --newline
  -s, --show-filename
  -c, --colorless
  -d, --delete                     deletes portions of input-data matching the given expression
  -l, --files-with-matches
  -C, --context <NUM>
  -g, --group-color <GROUP_COLOR>  [default: 220]
  -m, --match-color <MATCH_COLOR>  [default: 154]
  -h, --help                       Print help
  -V, --version                    Print version
```
