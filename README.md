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
        <img src="https://img.shields.io/crates/v/bat.svg?colorB=319e8c" alt="Version">
    </a>
    <a href="https://github.com/wfxr/csview/releases">
        <img src="https://img.shields.io/badge/platform-%20Linux%20|%20OSX%20|%20Win%20|%20ARM-orange.svg" alt="Platform"/>
    </a>
</p>

<img src="https://raw.githubusercontent.com/wfxr/i/master/csview-screenshot.png" />

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
| Year | Make  | Model                                  | Description            | Price   |
+------+-------+----------------------------------------+------------------------+---------+
| 1997 | Ford  | E350                                   | ac, abs, moon          | 3000.00 |
| 1999 | Chevy | Venture "Extended Edition"             |                        | 4900.00 |
| 1999 | Chevy | Venture "Extended Edition, Very Large" |                        | 5000.00 |
| 1996 | Jeep  | Grand Cherokee                         | MUST SELL!             | 4799.00 |
|      |       |                                        | air, moon roof, loaded |         |
+------+-------+----------------------------------------+------------------------+---------+
```

Run `csview --help` to view detailed usage.

### Benchmark

Compared with `csvlook` provided by [csvkit](https://github.com/wireservice/csvkit/tree/1.0.5):

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

[sample.csv](https://gist.github.com/wfxr/567e890d4db508b3c7630a96b703a57e)

### License

`csview` is distributed under the terms of both the MIT License and the Apache License 2.0.

See the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files for license details.
