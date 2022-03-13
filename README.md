# uni

`uni` is a command to make unique text like `uniq` command.

The difference with `uniq` is that it does not require sorting of the input data.

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

## Note

`awk` can do the same. 😉

```sh
cat text | '!a[$0]++{print}'
```
