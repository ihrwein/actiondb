# pdb2adb

A command line utility to convert patterndb patterns to actiondb format.

## Required libraries
### Ubuntu 14.04
```
sudo apt-get install libxml-twig-perl
sudo apt-get install libgetopt-long-descriptive-perl
sudo apt-get install libjson-maybexs-perl
sudo apt-get install libdata-uuid-perl
```

## Usage

```
pdb2adb [-aePp] [long options...] <some-arg>
    -a --actions     toggle dump actions
    -p --patterns    toggle dump patterns
    -e --examples    toggle dump examples
    -P --program     toggle prepend '$PROGRAM: '
```
