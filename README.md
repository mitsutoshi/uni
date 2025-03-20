# uni

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

`uni` is a command to make unique text like `uniq` command.

The difference from `uniq` is that `uni` does not require sorting of the input data before running commnad.

`uni` can output text in the same order as when you input them.

## How to use

`uni` receives text from `stdin` and unique it.

```
% cat text
a
c
b
c

% cat text | uni
a
c
b
```

## How to install

If you use MacOS with Apple Silicon, you can download a binary from the Github.

https://github.com/mitsutoshi/uni/releases

Or you can get it by using Homebrew.

```sh
brew tap mitsutoshi/uni
brew install mitsutoshi/uni/uni
```

If you use other OS, please clone this repository to your computer and make a binary for your OS.

## Note

`awk` can do the same thing. 😉

```sh
cat text | awk '!a[$0]++{print}'
```
