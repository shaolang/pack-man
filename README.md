# pack-man [![Build Status](https://travis-ci.org/shaolang/pack-man.png)](https://travis-ci.org/shaolang/pack-man)

pack-man is a command-line tool that packs the Postscript file generated
by [PDFtk][pdftk] or [Ghostscript][gs] for easier deciphering.

```postscript
    580.4 200 Td
    ( Hello )
    [3.9754
    0
    3.9754
    0
    3.9754
    0
    3.9754
    0
    3.9754
    0] Tj
```

The snippet above shows the Postscript instructions in specifying the offsets
(via `Td`) and the outputting of the text ` Hello ` (via `Tj`). pack-man
compresses to an easier to decipher output as follows:

```postscript
    580.4 200 Td
    ( Hello ) [3.9754 0 3.9754 0 3.9754 0 3.9754 0 3.9754 0] Tj
```

While the example is contrived, pack-man's strength is amplified when the
target string to display is longer than `( Hello )` as the length of the
numbers in brackets increases.

## Usage

First, generate the Postscript file

    $ pdftops -paper A4 src [dest]

where `src` is the name of the PDF file. You can also specify the name of
the generated Postscript file. If you omit it, `pdftops` will simply
use the name of the source file with `.ps` as the dest file name.

Then invoke pack-man as follows:

    $ pack-man src [dest]

Where `src` is the name of the generated Postscript file, and optionally
`dest` is the name of the packed file, or pack-man prepend the `src`
file name with a `.ps` extension.

Amend the packed Postscript as necessary. To convert the amended
Postscript file to PDF:

    $ ps2pdf src [dest]

where `src` is the name of the amended Postscript file, and `dest` is the
optional dest file name.

## Genesis

I wrote the original version in Clojure; if you refer to use that version
instead, use the commit tagged 0.1.0.

## License

Copyright © 2016 Shaolang Ai

Distributed under the MIT License

[pdftk]: https://www.pdflabs.com/tools/pdftk-the-pdf-toolkit
[gs]: http://www.ghostscript.com
