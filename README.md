# Revise Cross Parameters

Rust library for estimating the cost of a forgery attack on CROSS involving the fixed-weight distribution of the second challenge.

## Installation

To install clone the repository and build with cargo (release mode is recommended):

```sh
cargo build --release
```

## Usage

The library has a command line interface, which can be called via `cargo run`. The CLI requires specifying values for $p$ (the prime order of finite field $\mathbb{F}_p$), $t$ (the number of parallel repetitions) and $\omega$ (the fixed-weight parameter for the second challenge).

```sh
$ cargo run --release -- --help
Usage: revise_cross_parameters_cli [OPTIONS] -p <P> -t <T> -w <W>

Options:
  -p <P>                   Prime order of finite field Fp
  -t <T>                   Number of parallel repetitions
  -w <W>                   Fixed-weight parameter for the second challenge
      --threads <THREADS>  Number of threads (default all)
      --quiet              Do not show a progress bar
  -h, --help               Print help
  -V, --version            Print version

```

## Results

| Set                          | p   | t   | w   | Complexity (CROSS) | t\* | Complexity (Our) | t\* | alpha |
| ---------------------------- | --- | --- | --- | ------------------ | --- | ---------------- | --- | ----- |
| CROSS-R-SDP 1 fast           | 127 | 163 | 85  | 128.06             | 35  | 128.05           | 35  | 86    |
| CROSS-R-SDP 1 balanced       | 127 | 252 | 212 | 128.01             | 40  | **120.46**       | 38  | 227   |
| CROSS-R-SDP 1 small          | 127 | 960 | 938 | 128.00             | 65  | **97.48**        | 55  | 960   |
| CROSS-R-SDP 3 fast           | 127 | 245 | 127 | 192.08             | 52  | 192.05           | 52  | 128   |
| CROSS-R-SDP 3 balanced       | 127 | 398 | 340 | 192.07             | 61  | **179.67**       | 59  | 365   |
| CROSS-R-SDP 3 small          | 127 | 945 | 907 | 192.02             | 83  | **156.37**       | 73  | 944   |
| CROSS-R-SDP 5 fast           | 127 | 327 | 169 | 256.06             | 70  | 256.03           | 70  | 171   |
| CROSS-R-SDP 5 balanced       | 127 | 327 | 169 | 256.01             | 81  | **240.82**       | 78  | 459   |
| CROSS-R-SDP 5 small          | 127 | 968 | 912 | 255.22             | 101 | **217.15**       | 91  | 957   |
| CROSS-R-SDP ($G$) 1 fast     | 509 | 153 | 79  | 128.06             | 24  | 128.06           | 24  | 79    |
| CROSS-R-SDP ($G$) 1 balanced | 509 | 243 | 206 | 128.13             | 27  | **122.72**       | 26  | 216   |
| CROSS-R-SDP ($G$) 1 small    | 509 | 871 | 850 | 128.01             | 38  | **108.22**       | 34  | 867   |
| CROSS-R-SDP ($G$) 3 fast     | 509 | 230 | 123 | 192.03             | 37  | 191.98           | 37  | 125   |
| CROSS-R-SDP ($G$) 3 balanced | 509 | 255 | 176 | 192.03             | 37  | 189.83           | 37  | 184   |
| CROSS-R-SDP ($G$) 3 small    | 509 | 949 | 914 | 192.03             | 53  | **167.56**       | 48  | 937   |
| CROSS-R-SDP ($G$) 5 fast     | 509 | 306 | 157 | 256.01             | 49  | 256.00           | 49  | 158   |
| CROSS-R-SDP ($G$) 5 balanced | 509 | 356 | 257 | 256.08             | 51  | 252.70           | 50  | 270   |
| CROSS-R-SDP ($G$) 5 small    | 509 | 996 | 945 | 256.03             | 66  | **228.58**       | 61  | 974   |

## Idea

See [scripts/attack.ipynb](scripts/attack.ipynb) for a high-level description of the forgery.