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
* Memory efficient.
* Correctly align [CJK](https://en.wikipedia.org/wiki/CJK_characters) and emoji characters.
* Support `tsv` and custom delimiters.
* Support different styles, including markdown table.

### Usage
```
$ cat example.csv
Year,Make,Model,Description,Price
1997,Ford,E350,"ac, abs, moon",3000.00
1999,Chevy,"Venture ""Extended Edition""","",4900.00
1999,Chevy,"Venture ""Extended Edition, Large""",,5000.00
1996,Jeep,Grand Cherokee,"MUST SELL! air, moon roof",4799.00

$ csview example.csv
┌──────┬───────┬───────────────────────────────────┬───────────────────────────┬─────────┐
│ Year │ Make  │ Model                             │ Description               │ Price   │
├──────┼───────┼───────────────────────────────────┼───────────────────────────┼─────────┤
│ 1997 │ Ford  │ E350                              │ ac, abs, moon             │ 3000.00 │
│ 1999 │ Chevy │ Venture "Extended Edition"        │                           │ 4900.00 │
│ 1999 │ Chevy │ Venture "Extended Edition, Large" │                           │ 5000.00 │
│ 1996 │ Jeep  │ Grand Cherokee                    │ MUST SELL! air, moon roof │ 4799.00 │
└──────┴───────┴───────────────────────────────────┴───────────────────────────┴─────────┘

$ head /etc/passwd | csview -H -d:
┌────────────────────────┬───┬───────┬───────┬────────────────────────────┬─────────────────┐
│ root                   │ x │ 0     │ 0     │                            │ /root           │
│ bin                    │ x │ 1     │ 1     │                            │ /               │
│ daemon                 │ x │ 2     │ 2     │                            │ /               │
│ mail                   │ x │ 8     │ 12    │                            │ /var/spool/mail │
│ ftp                    │ x │ 14    │ 11    │                            │ /srv/ftp        │
│ http                   │ x │ 33    │ 33    │                            │ /srv/http       │
│ nobody                 │ x │ 65534 │ 65534 │ Nobody                     │ /               │
│ dbus                   │ x │ 81    │ 81    │ System Message Bus         │ /               │
│ systemd-journal-remote │ x │ 981   │ 981   │ systemd Journal Remote     │ /               │
│ systemd-network        │ x │ 980   │ 980   │ systemd Network Management │ /               │
└────────────────────────┴───┴───────┴───────┴────────────────────────────┴─────────────────┘
```

Run `csview --help` to view detailed usage.

### Installation

#### On Arch Linux

`csview` is available in the Arch User Repository. To install it from [AUR](https://aur.archlinux.org/packages/csview):

```
yay -S csview
```

#### On macOS

You can install `csview` with Homebrew:

```
brew tap wfxr/csview
brew install csview
```

#### On NetBSD

`csview` is available from the main pkgsrc Repositories. To install simply run

```
pkgin install csview
```

or, if you prefer to build from source using [pkgsrc](https://pkgsrc.se/textproc/csview) on any of the supported platforms:

```
cd /usr/pkgsrc/textproc/csview
make install
```

#### On Windows

You can install `csview` with [Scoop](https://scoop.sh/):
```
scoop install csview
```

#### From binaries

Pre-built versions of `csview` for various architectures are available at [Github release page](https://github.com/wfxr/csview/releases).

*Note that you can try the `musl` version (which is statically-linked) if runs into dependency related errors.*

#### From source

`csview` is also published on [crates.io](https://crates.io). If you have Rust toolchains (nightly) installed you can use `cargo` to install it from source:

```
cargo install --locked csview
```

If you want the latest version, clone this repository and run `cargo build --release`.

### Benchmark

- [small.csv](https://gist.github.com/wfxr/567e890d4db508b3c7630a96b703a57e#file-action-csv) (10 rows, 4 cols, 695 bytes):

|                                           Tool                                           | Command                   | Mean Time |  Min Time |    Memory |
|:----------------------------------------------------------------------------------------:|---------------------------|----------:|----------:|----------:|
|                   [xsv](https://github.com/BurntSushi/xsv/tree/0.13.0)                   | `xsv table small.csv`     |     2.0ms |     1.8ms |     3.9mb |
|  [csview](https://github.com/wfxr/csview/tree/90ff90e26c3e4c4c37818d717555b3e8f90d27e3)  | `csview small.csv`        | **0.3ms** | **0.1ms** | **2.4mb** |
| [column](https://github.com/util-linux/util-linux/blob/stable/v2.37/text-utils/column.c) | `column -s, -t small.csv` |     1.3ms |     1.1ms | **2.4mb** |
|                [csvlook](https://github.com/wireservice/csvkit/tree/1.0.6)               | `csvlook small.csv`       |   148.1ms |   142.4ms |    27.3mb |

- [medium.csv](https://gist.github.com/wfxr/567e890d4db508b3c7630a96b703a57e#file-sample-csv) (10,000 rows, 10 cols, 624K bytes):

|                                           Tool                                           | Command                   |  Mean Time |   Min Time |    Memory |
|:----------------------------------------------------------------------------------------:|---------------------------|-----------:|-----------:|----------:|
|                   [xsv](https://github.com/BurntSushi/xsv/tree/0.13.0)                   | `xsv table medium.csv`    |     0.031s |     0.029s |     4.4mb |
|  [csview](https://github.com/wfxr/csview/tree/90ff90e26c3e4c4c37818d717555b3e8f90d27e3)  | `csview medium.csv`       | **0.017s** | **0.016s** | **2.8mb** |
| [column](https://github.com/util-linux/util-linux/blob/stable/v2.37/text-utils/column.c) | `column -s, -t small.csv` |     0.052s |     0.050s |     9.9mb |
|                [csvlook](https://github.com/wireservice/csvkit/tree/1.0.6)               | `csvlook medium.csv`      |     2.664s |     2.617s |    46.8mb |

- `large.csv` (1,000,000 rows, 10 cols, 61M bytes, generated by concatenating [medium.csv](https://gist.github.com/wfxr/567e890d4db508b3c7630a96b703a57e#file-sample-csv) 100 times):

|                                           Tool                                           | Command                   |  Mean Time |   Min Time |    Memory |
|:----------------------------------------------------------------------------------------:|---------------------------|-----------:|-----------:|----------:|
|                   [xsv](https://github.com/BurntSushi/xsv/tree/0.13.0)                   | `xsv table large.csv`     |     2.912s |     2.820s |     4.4mb |
|  [csview](https://github.com/wfxr/csview/tree/90ff90e26c3e4c4c37818d717555b3e8f90d27e3)  | `csview large.csv`        | **1.686s** | **1.665s** | **2.8mb** |
| [column](https://github.com/util-linux/util-linux/blob/stable/v2.37/text-utils/column.c) | `column -s, -t small.csv` |     5.777s |     5.759s |   767.6mb |
|                [csvlook](https://github.com/wireservice/csvkit/tree/1.0.6)               | `csvlook large.csv`       |    20.665s |    20.549s |  1105.7mb |

### F.A.Q.

---
#### We already have [xsv](https://github.com/BurntSushi/xsv), why not contribute to it but build a new tool?

`xsv` is great. But it's aimed for analyzing and manipulating csv data.
`csview` is designed for formatting and viewing. See also: [xsv/issues/156](https://github.com/BurntSushi/xsv/issues/156)

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
