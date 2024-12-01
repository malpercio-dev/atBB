# atBB

A BB-style forum, on the ATmosphere!

## Usage

To build the container and run the application:

```bash
docker build -t atbb .
docker run --rm -it atbb --nsid "app.bsky.feed.post" --did "did:web:malpercio.dev"
```

## Contributing

All contributors are welcome! Broadly, the goal of this repository is to build out a
BB-style forum (such as phpBB) on ATProto.

This will likely involve building out, at mimumum,
a lexicon, an AppView, and designed with flexibility and ease in deployment in mind so that it is
simple for admins to set up a new atBB forum.
