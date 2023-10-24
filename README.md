## Alpha bleeding

A tool used to fix white border issues by doing alpha bleeding.

Similar project: 
 - https://github.com/urraka/alpha-bleeding
 - https://github.com/dmi7ry/alpha-bleeding-d

### Use as a CLI

```
Usage: alpha_bleeding [INPUT] [OUPTUT]

Arguments:
  [INPUT]   Input image path to be processed
  [OUPTUT]  Output image path, if not provided, replace the input image
```

### Use as a lib

```rs
use alpha_bleeding::alpha_bleeding;
alpha_bleeding("origin.png", "fixed.png");
```