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
│ Year │ Make  │               Model               │        Description        │  Price  │
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

`csview` is also published on [crates.io](https://crates.io). If you have latest Rust toolchains installed you can use `cargo` to install it from source:

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


## 🌐 Web Resources & Aesthetic Symbols Index
- [SYM 2745](https://aestheticsymbols.io/symbol/sym-2745/)
- [ZODIAC CELESTIAL](https://aestheticsymbols.io/ru/zodiac-celestial/)
- [SYM 26CD](https://aestheticsymbols.io/symbol/sym-26cd/)
- [SYM 1F92F](https://aestheticsymbols.io/symbol/sym-1f92f/)
- [ROBLOX NAMES](https://aestheticsymbols.io/ru/roblox-names/)
- [SYM 26FC](https://aestheticsymbols.io/symbol/sym-26fc/)
- [CHEERING FIGHTING FIST KAOMOJI](https://aestheticsymbols.io/symbol/cheering-fighting-fist-kaomoji/)
- [SYM 1F929](https://aestheticsymbols.io/symbol/sym-1f929/)
- [SYM 2664](https://aestheticsymbols.io/symbol/sym-2664/)
- [VI](https://aestheticsymbols.io/vi/)
- [WHITE STAR](https://aestheticsymbols.io/symbol/white-star/)
- [BLACK CENTRE STAR](https://aestheticsymbols.io/symbol/black-centre-star/)
- [SYM 26D8](https://aestheticsymbols.io/symbol/sym-26d8/)
- [SYM 1F612](https://aestheticsymbols.io/symbol/sym-1f612/)
- [SYM 1F60B](https://aestheticsymbols.io/symbol/sym-1f60b/)
- [SYM 260B](https://aestheticsymbols.io/symbol/sym-260b/)
- [SIX POINTED BLACK STAR](https://aestheticsymbols.io/symbol/six-pointed-black-star/)
- [SYM 2666](https://aestheticsymbols.io/symbol/sym-2666/)
- [SYM 2677](https://aestheticsymbols.io/symbol/sym-2677/)
- [FLORAL HEART VINE](https://aestheticsymbols.io/symbol/floral-heart-vine/)
- [WHITE HEART](https://aestheticsymbols.io/symbol/white-heart/)
- [TABLE FLIP RAGE KAOMOJI](https://aestheticsymbols.io/symbol/table-flip-rage-kaomoji/)
- [SYM 1D4A4](https://aestheticsymbols.io/symbol/sym-1d4a4/)
- [RU](https://aestheticsymbols.io/ru/)
- [SYM 1F922](https://aestheticsymbols.io/symbol/sym-1f922/)
- [SYM 1F924](https://aestheticsymbols.io/symbol/sym-1f924/)
- [ZODIAC CELESTIAL](https://aestheticsymbols.io/pt/zodiac-celestial/)
- [DISCORD STATUS](https://aestheticsymbols.io/es/discord-status/)
- [CUPID FEATHERY ARROW](https://aestheticsymbols.io/symbol/cupid-feathery-arrow/)
- [SYM 1D44F](https://aestheticsymbols.io/symbol/sym-1d44f/)
- [MUSIC WEATHER](https://aestheticsymbols.io/ru/music-weather/)
- [DAGGER BLADE](https://aestheticsymbols.io/symbol/dagger-blade/)
- [SYM 1D444](https://aestheticsymbols.io/symbol/sym-1d444/)
- [EIGHT POINTED STAR](https://aestheticsymbols.io/symbol/eight-pointed-star/)
- [SYM 26E8](https://aestheticsymbols.io/symbol/sym-26e8/)
- [SYM 2687](https://aestheticsymbols.io/symbol/sym-2687/)
- [SYM 2633](https://aestheticsymbols.io/symbol/sym-2633/)
- [SYM 262B](https://aestheticsymbols.io/symbol/sym-262b/)
- [SYM 1FAE1](https://aestheticsymbols.io/symbol/sym-1fae1/)
- [HEARTS](https://aestheticsymbols.io/hearts/)
- [SYM 1F619](https://aestheticsymbols.io/symbol/sym-1f619/)
- [SYM 26DE](https://aestheticsymbols.io/symbol/sym-26de/)
- [HEARTS](https://aestheticsymbols.io/pt/hearts/)
- [FOUR POINT STAR SPARKLE](https://aestheticsymbols.io/symbol/four-point-star-sparkle/)
- [SYM 1F61A](https://aestheticsymbols.io/symbol/sym-1f61a/)
- [CROSSED SWORDS](https://aestheticsymbols.io/symbol/crossed-swords/)
- [NATURE FLOWERS](https://aestheticsymbols.io/nature-flowers/)
- [STARRY LOVE AURA](https://aestheticsymbols.io/symbol/starry-love-aura/)
- [SYM 265D](https://aestheticsymbols.io/symbol/sym-265d/)
- [SYM 1F914](https://aestheticsymbols.io/symbol/sym-1f914/)
- [SYM 1D44C](https://aestheticsymbols.io/symbol/sym-1d44c/)
- [SYM 26F0](https://aestheticsymbols.io/symbol/sym-26f0/)
- [ROBLOX NAMES](https://aestheticsymbols.io/ja/roblox-names/)
- [SYM 1F925](https://aestheticsymbols.io/symbol/sym-1f925/)
- [TIKTOK CAPTIONS](https://aestheticsymbols.io/es/tiktok-captions/)
- [SYM 1D466](https://aestheticsymbols.io/symbol/sym-1d466/)
- [EIGHT POINTED BLACK STAR](https://aestheticsymbols.io/symbol/eight-pointed-black-star/)
- [SYM 1D418](https://aestheticsymbols.io/symbol/sym-1d418/)
- [SYM 273E](https://aestheticsymbols.io/symbol/sym-273e/)
- [SYM 1D43E](https://aestheticsymbols.io/symbol/sym-1d43e/)
- [SYM 2679](https://aestheticsymbols.io/symbol/sym-2679/)
- [ROTATED FLORAL HEART](https://aestheticsymbols.io/symbol/rotated-floral-heart/)
- [SYM 2681](https://aestheticsymbols.io/symbol/sym-2681/)
- [BLACK HEART](https://aestheticsymbols.io/symbol/black-heart/)
- [SYM 1D445](https://aestheticsymbols.io/symbol/sym-1d445/)
- [SYM 260E](https://aestheticsymbols.io/symbol/sym-260e/)
- [SYM 1D461](https://aestheticsymbols.io/symbol/sym-1d461/)
- [CYBER PHANTOM GLYPH](https://aestheticsymbols.io/symbol/cyber-phantom-glyph/)
- [SYM 1D460](https://aestheticsymbols.io/symbol/sym-1d460/)
- [SYM 2660](https://aestheticsymbols.io/symbol/sym-2660/)
- [SYM 2683](https://aestheticsymbols.io/symbol/sym-2683/)
- [FREEFIRE NAMES](https://aestheticsymbols.io/ru/freefire-names/)
- [SYM 1D453](https://aestheticsymbols.io/symbol/sym-1d453/)
- [SYM 26D2](https://aestheticsymbols.io/symbol/sym-26d2/)
- [SYM 1D433](https://aestheticsymbols.io/symbol/sym-1d433/)
- [SYM 1D415](https://aestheticsymbols.io/symbol/sym-1d415/)
- [SYM 2636](https://aestheticsymbols.io/symbol/sym-2636/)
- [FLUTTERING BUTTERFLY](https://aestheticsymbols.io/symbol/fluttering-butterfly/)
- [GEMINI ZODIAC TWINS](https://aestheticsymbols.io/symbol/gemini-zodiac-twins/)
- [SYM 26A7](https://aestheticsymbols.io/symbol/sym-26a7/)
- [BRACKETS](https://aestheticsymbols.io/ru/brackets/)
- [SYM 267B](https://aestheticsymbols.io/symbol/sym-267b/)
- [SYM 1F921](https://aestheticsymbols.io/symbol/sym-1f921/)
- [OUTLINED STAR](https://aestheticsymbols.io/symbol/outlined-star/)
- [SYM 1D43D](https://aestheticsymbols.io/symbol/sym-1d43d/)
- [SYM 2639 FE0F](https://aestheticsymbols.io/symbol/sym-2639-fe0f/)
- [SUPER SHY BLUSHING KAOMOJI](https://aestheticsymbols.io/symbol/super-shy-blushing-kaomoji/)
- [RIGHT HEAVY BRACKET BOX](https://aestheticsymbols.io/symbol/right-heavy-bracket-box/)
- [SYM 2764 FE0F](https://aestheticsymbols.io/symbol/sym-2764-fe0f/)
- [STARRY ELEVATION AURA](https://aestheticsymbols.io/symbol/starry-elevation-aura/)
- [SYM 1F609](https://aestheticsymbols.io/symbol/sym-1f609/)
- [SYM 2684](https://aestheticsymbols.io/symbol/sym-2684/)
- [SYM 1F928](https://aestheticsymbols.io/symbol/sym-1f928/)
- [SYM 1F627](https://aestheticsymbols.io/symbol/sym-1f627/)
- [SYM 26C6](https://aestheticsymbols.io/symbol/sym-26c6/)
- [SYM 2659](https://aestheticsymbols.io/symbol/sym-2659/)
- [GAMING WEAPONS](https://aestheticsymbols.io/ja/gaming-weapons/)
- [SYM 1F978](https://aestheticsymbols.io/symbol/sym-1f978/)
- [SYM 268F](https://aestheticsymbols.io/symbol/sym-268f/)
- [SYM 1D463](https://aestheticsymbols.io/symbol/sym-1d463/)
- [INSTAGRAM BIO](https://aestheticsymbols.io/es/instagram-bio/)
- [SYM 1D44B](https://aestheticsymbols.io/symbol/sym-1d44b/)
- [SYM 1F624](https://aestheticsymbols.io/symbol/sym-1f624/)
- [LITTLE CAT PAWS KAOMOJI](https://aestheticsymbols.io/symbol/little-cat-paws-kaomoji/)
- [SYM 1D472](https://aestheticsymbols.io/symbol/sym-1d472/)
- [SYM 1D447](https://aestheticsymbols.io/symbol/sym-1d447/)
- [SYM 1F636 200D 1F32B FE0F](https://aestheticsymbols.io/symbol/sym-1f636-200d-1f32b-fe0f/)
- [SYM 267D](https://aestheticsymbols.io/symbol/sym-267d/)
- [SYM 1F623](https://aestheticsymbols.io/symbol/sym-1f623/)
- [SYM 1D437](https://aestheticsymbols.io/symbol/sym-1d437/)
- [SYM 26F4](https://aestheticsymbols.io/symbol/sym-26f4/)
- [SYM 26E7](https://aestheticsymbols.io/symbol/sym-26e7/)
- [SYM 1D41B](https://aestheticsymbols.io/symbol/sym-1d41b/)
- [GREEK PSI TRIDENT](https://aestheticsymbols.io/symbol/greek-psi-trident/)
- [SYM 1D468](https://aestheticsymbols.io/symbol/sym-1d468/)
- [FREEFIRE NAMES](https://aestheticsymbols.io/pt/freefire-names/)
- [SYM 2657](https://aestheticsymbols.io/symbol/sym-2657/)
- [SYM 262A](https://aestheticsymbols.io/symbol/sym-262a/)
- [SYM 2689](https://aestheticsymbols.io/symbol/sym-2689/)
- [SYM 26C5](https://aestheticsymbols.io/symbol/sym-26c5/)
- [RIGHT WING CLAN FLARE](https://aestheticsymbols.io/symbol/right-wing-clan-flare/)
- [BORDERS DIVIDERS](https://aestheticsymbols.io/es/borders-dividers/)
- [SYM 1F499](https://aestheticsymbols.io/symbol/sym-1f499/)
- [NATURE FLOWERS](https://aestheticsymbols.io/ru/nature-flowers/)
- [SYM 1F644](https://aestheticsymbols.io/symbol/sym-1f644/)
- [TELUGU RIBBON BOWLET](https://aestheticsymbols.io/symbol/telugu-ribbon-bowlet/)
- [SYM 1FAE2](https://aestheticsymbols.io/symbol/sym-1fae2/)
- [SYM 2722](https://aestheticsymbols.io/symbol/sym-2722/)
- [SYM 1F639](https://aestheticsymbols.io/symbol/sym-1f639/)
- [RINGED PLANET SATURN](https://aestheticsymbols.io/symbol/ringed-planet-saturn/)
