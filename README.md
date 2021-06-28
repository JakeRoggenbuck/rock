# rock

# Usage
| short | long        | description                                 | example                           |
|-------|-------------|---------------------------------------------|-----------------------------------|
| `-r`  | `--replace` | replace a certain string with another       | `rock --replace "~" "/home/jake"` |
| `-s`  | `--split`   | split into multiple lines after a character | `rock --split ,`                  |

# Examples

```
echo $PATH | rock -s : | rock -r "/home/jake" "~"

echo "~/Downloads,~/Documents,~/Repos/rock" | rock --replace "~" "/home/jake"

echo "~/Downloads,~/Documents,~/Repos/rock" | rock --split , | rock --replace "~" "/home/jake"
```

# TODO Feature
```
-f --filter will filter out something
-b --block will completely block out something
-o --only will only show something
-p --prepend something to the begining
-a --append something the end
```
