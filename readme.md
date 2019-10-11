# Runik

An improved Rust written Uniq. Based on [Count](https://github.com/juliangehring/count).

Runik removes duplicates from text lists. Unlike Uniq it can be told to only keep entries that occur at least x times.

## Examples

```shell
# call for help
runik -h

# pipe
cat whatever.txt | runik

# as cli tool
runik -t 5 whatever.txt
```