<h1 align="center">📠 csview</h1>
<p align="center">
    <em>A high performance csv viewer with cjk/emoji support.</em>
</p>

<p align="center">
    <a href="https://github.com/wfxr/csview/actions?query=workflow%3ACICD">
        <img src="https://github.com/wfxr/csview/workflows/CICD/badge.svg" alt="CICD"/>
    </a>
    <img src="https://img.shields.io/crates/l/csview.svg" alt="License"/>
    <a href="https://crates.io/crates/csview">
        <img src="https://img.shields.io/crates/v/csview.svg?colorB=319e8c" alt="Version">
    </a>
    <a href="https://github.com/wfxr/csview/releases">
        <img src="https://img.shields.io/badge/platform-%20Linux%20|%20OSX%20|%20Win%20|%20ARM-orange.svg" alt="Platform"/>
    </a>
</p>

<img src="https://raw.githubusercontent.com/wfxr/i/master/csview-screenshot.png" />

### Features

* Small and *fast* (see [benchmarks](#benchmark) below).
* Correctly handles CJK characters and emoji.
* Support different styles.
* Support `tsv` and custom delimiters.
* Able to generate markdown table (with `--style markdown` option).

### Usage
```
$ cat example.csv
Year,Make,Model,Description,Price
1997,Ford,E350,"ac, abs, moon",3000.00
1999,Chevy,"Venture ""Extended Edition""","",4900.00
1999,Chevy,"Venture ""Extended Edition, Very Large""",,5000.00
1996,Jeep,Grand Cherokee,"MUST SELL!
air, moon roof, loaded",4799.00

$ csview example.csv
+------+-------+----------------------------------------+------------------------+---------+
| Year | Make | Model | Description | Price |
+------+-------+----------------------------------------+------------------------+---------+
| 1997 | Ford  | E350                                   | ac, abs, moon          | 3000.00 |
| 1999 | Chevy | Venture "Extended Edition"             |                        | 4900.00 |
| 1999 | Chevy | Venture "Extended Edition, Very Large" |                        | 5000.00 |
| 1996 | Jeep  | Grand Cherokee                         | MUST SELL!             | 4799.00 |
|      |       |                                        | air, moon roof, loaded |         |
+------+-------+----------------------------------------+------------------------+---------+

$ head -n10 /etc/passwd | csview --no-headers -d:
+------------------------+---+-------+-------+----------------------------+-----------------+
| root                   | x | 0     | 0     |                            | /root           |
| bin                    | x | 1     | 1     |                            | /               |
| daemon                 | x | 2     | 2     |                            | /               |
| mail                   | x | 8     | 12    |                            | /var/spool/mail |
| ftp                    | x | 14    | 11    |                            | /srv/ftp        |
| http                   | x | 33    | 33    |                            | /srv/http       |
| nobody                 | x | 65534 | 65534 | Nobody                     | /               |
| dbus                   | x | 81    | 81    | System Message Bus         | /               |
| systemd-journal-remote | x | 982   | 982   | systemd Journal Remote     | /               |
| systemd-network        | x | 981   | 981   | systemd Network Management | /               |
+------------------------+---+-------+-------+----------------------------+-----------------+
```

Run `csview --help` to view detailed usage.

### Installation

#### From binaries

Prebuilt versions of `csview` for various architectures are available at [Github release page](https://github.com/wfxr/csview/releases).

*Note that you can try the `musl` version (which is statically-linked) if runs into dependency related errors.*

#### From source

`csview` is also published on [crates.io](https://crates.io). If you have Rust toolchains (1.40 or above) installed you can use `cargo` to install it from source:

```
cargo install --locked csview
```

If you want the latest version, clone this repository and run `cargo build --release`.

### Benchmark

Compared with `csvlook` provided by [csvkit](https://github.com/wireservice/csvkit/tree/1.0.5):

- [sample.csv](https://gist.github.com/wfxr/567e890d4db508b3c7630a96b703a57e#file-sample-csv) (10000 rows, 10 cols, 624K size):

```
Benchmark #1: csvlook sample.csv
  Time (mean ± σ):      4.010 s ±  0.100 s    [User: 3.945 s, System: 0.051 s]
  Range (min … max):    3.911 s …  4.249 s    10 runs

Benchmark #2: csview sample.csv
  Time (mean ± σ):      46.5 ms ±   2.3 ms    [User: 39.7 ms, System: 6.5 ms]
  Range (min … max):    44.0 ms …  59.4 ms    59 runs

Summary
  'csview sample.csv' ran
   86.32 ± 4.83 times faster than 'csvlook sample.csv'
```

- [action.csv](https://gist.github.com/wfxr/567e890d4db508b3c7630a96b703a57e#file-action-csv) (10 rows, 4 cols, 1K size):
```
Benchmark #1: csvlook action.csv
  Time (mean ± σ):     316.5 ms ±   5.2 ms    [User: 284.8 ms, System: 35.0 ms]
  Range (min … max):   309.3 ms … 326.2 ms    10 runs

Benchmark #2: csview action.csv
  Time (mean ± σ):       0.7 ms ±   0.2 ms    [User: 0.8 ms, System: 0.7 ms]
  Range (min … max):     0.4 ms …   1.6 ms    933 runs

Summary
  'csview action.csv' ran
  461.25 ± 109.34 times faster than 'csvlook action.csv'
```

### F.A.Q.

---
#### We already have [xsv](https://github.com/BurntSushi/xsv), why not contribute to it but build a new tool?

`xsv` is an awesome csv tookit. It's aimed for analyzing and manipulating csv data.
`csview` is designed for formatting and viewing. Their relationship is like `awk` and `column`.

The author of `xsv` may have the similar views with me: https://github.com/BurntSushi/xsv/issues/156

---
#### Things look weird if the width of table is more than the width of terminal window.

Use pager less with -S option: `csview a.csv | less -S` so you can scroll screen horizontally.
Or use [xsv](https://github.com/BurntSushi/xsv) to filter out the columns you don't care then piped to csview.

---
#### I encountered UTF-8 related errors, how to solve it?

The file may use a non-UTF8 encoding. You can check the file encoding using `file` command:

```
$ file -i a.csv
a.csv: application/csv; charset=iso-8859-1
```
And then convert it to `utf8`:

```
$ iconv -f iso-8859-1 -t UTF8//TRANSLIT a.csv -o b.csv
$ csview b.csv
```

Or do it in place:

```
$ iconv -f iso-8859-1 -t UTF8//TRANSLIT a.csv | csview
```

### Credits

* [csv-rust](https://github.com/BurntSushi/rust-csv)
* [prettytable-rs](https://github.com/phsym/prettytable-rs)
* [structopt](https://github.com/TeXitoi/structopt)

### License

`csview` is distributed under the terms of both the MIT License and the Apache License 2.0.

See the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files for license details.
