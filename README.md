# Aur
[![Test]

[![Test](https://github.com/snltd/aur-rs/actions/workflows/test.yml/badge.svg)](https://github.com/snltd/aur-rs/actions/workflows/test.yml)

I have a pretty large collection of digital music (and even bigger physical
one) and I like to keep it organized. *Really* organized.

This program helps with that, pulling together a dozen or so smaller scripts
written over the last twenty-ish years in various languages, adding tests and
a uniform interface.

Rules and assumptions are:

* FLACs are in `/storage/flac`; MP3s are in `/storage/mp3`. Every FLAC exists
  as an MP3, but not vice-versa.
* Albums are under `albums/abc` etc; EPs and singles under `eps/`; loose
  tracks under `tracks/`. Stuff to be processed and filed is under `new/`.
* Audio files are named `nn.artist.title.suffix`. nn is a zero-padded
  two-digit number. If the artist is "The" something, `the_` is removed from
  the filename.
* Tags must be populated for artist, title, album, track number, genre and
  year. Any other tags are removed.
* FLAC albums have artwork stored as `front.jpg`, and no bigger than 700x700
  pixels. MP3s have no artwork. Embedded artwork is removed.
* Capitalisation of titles is broadly in line with
  [this](https://www.ox.ac.uk/sites/files/oxford/Style%20Guide%20HT2016.pdf)
* Files not suffixed `flac` or `mp3` are silently ignored. (Expect by 
  `lintdir`). 
* Hitting a file which looks like music but isn't stops the world.
* Loads of other finnicky little nitpicks peculiar to me.

Though native crates exist to encode/decode/transcode all the media types I'm 
interested in, I have chosen to shell out to LAME and the reference FLAC 
encoder. Though I am sure the Rust media libraries are excellent, they aren't
battle tested to anything like the same degree.

I haven't included any proper documentation, because `aur` is so strongly
opinionated that it will likely be of no use to anyone else in the world. If 
you really insist on trying it, `--help` ought to be enough.

But, there are thousands of alternatives which do not implement all my personal
preferences. Use one of those.

This is a Rust rewrite of [a Ruby program](https://github.com/snltd/aur).
