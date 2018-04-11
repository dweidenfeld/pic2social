# Pic2Social

## Description
This crate is a image upload for social media portals.

```
pic2social 0.1.0
Dominik Weidenfeld <dominik@sh0k.de>
A social media image uploader with active directory watching

USAGE:
    pic2social [OPTIONS] --directory <directory> --message <message>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --access_token <access_token>                   [env: P2S_ACCESS_TOKEN=]
        --access_token_secret <access_token_secret>     [env: P2S_ACCESS_TOKEN_SECRET=]
        --consumer_key <consumer_key>                   [env: P2S_CONSUMER_KEY=]
        --consumer_secret <consumer_secret>             [env: P2S_CONSUMER_SECRET=]
    -d, --directory <directory>                         [env: P2S_DIRECTORY=]
    -m, --message <message>                             [env: P2S_MESSAGE=]
    -p, --plugin <plugin>                               [env: P2S_PLUGIN=]  [default: twitter]
```

You can run it with
```
./pic2social -d pic/ -m "#myHashTag" -p twitter --consumer_key=... --consumer_secret=...
```

Then it will watch the pic folder and everytime a picture is added
to the directory it will be uploaded with the provided message.

## Cross Compilation
`cross build --target=x86_64-pc-windows-gnu`