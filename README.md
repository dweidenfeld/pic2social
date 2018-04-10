# Pic2Social

## Description
This crate is a image upload for social media portals.

```
pic2social 0.1.0
Dominik Weidenfeld <dominik@sh0k.de>
A social media image uploader with active directory watching

USAGE:
    pic2social --directory <directory> --message <message>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --directory <directory>
    -m, --message <message>
```

You can run it with
```
./pic2social -d pic/ -m "#myHashTag"
```

Then it will watch the pic folder and everytime a picture is added
to the directory it will be uploaded with the provided message.

## Cross Compilation
`cross build --target=x86_64-pc-windows-gnu`