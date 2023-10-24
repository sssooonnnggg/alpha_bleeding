## Alpha bleeding

A tool used to fix white border issues by doing alpha bleeding.

Similar project: 
 - https://github.com/urraka/alpha-bleeding
 - https://github.com/dmi7ry/alpha-bleeding-d

### Use as a CLI

```
Usage: alpha_bleeding [INPUT] [OUPTUT]

Arguments:
    [INPUT] The path of the image to be fixed.
    [OUTPUT] The path where the fixed image will be saved. If an output path is not provided, the original input image will be replaced with the fixed one.
```

### Use as a lib

```rs
use alpha_bleeding::alpha_bleeding;
alpha_bleeding("origin.png", "fixed.png");
```