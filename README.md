---
title: Rust numerical
author: DiaSird
fontsize: 11pt
header-includes:
  - \lstset{numbers=left, frame=trlb, frameround=tttt, breaklines=true}
  - \renewcommand{\lstlistingname}{Source Code}
---

Initial version: 2022/02/13
Revised: 2022/07/08

# Equation of Motion with Rust

---

## Contents

- [Rust Equation of Motion](#equation-of-motion-with-rust)
  - [Contents](#contents)
  - [Example](#example)
  - [Make usage](#make-usage)
  - [Used version](#used-version)
  - [License](#license)

## Example

Equation of motion ($x = x(t)$)

$m \frac{d^2x}{dt^2} + c \frac{dx}{dt} + kx = 0$

$x - t$ curve;

<p align="center">
<img src="docs\output.png", width="80%">

</p>

## Make usage

| Command    |      Description       |
| :--------- | :--------------------: |
| `make run` |       cargo run        |
| `make ch`  | (default:) cargo check |

If you are using windows, you can install the `make` command [here](http://gnuwin32.sourceforge.net/packages/make.htm).
(Click the `Setup` button at the top.)

## Used version

- rustc 1.58.1
- rustup 1.24.3

## License

- MIT license
