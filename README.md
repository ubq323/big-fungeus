# Big Fungeus
A [Funge-98](https://esolangs.org/wiki/funge-98) interpreter, written in Rust. Currently supports most of the base spec but is buggy and lacks more advanced features.

## Todo:
- Support all the features of the core specification
- Be fully [Mycology](https://github.com/Deewiant/Mycology/)-compliant wrt the core specification
    - currently we mainly are, but there are bugs relating to boundary checking and the stack-stack commands.
- support fingerprints
    - support fingerprints that require doing interesting things inside the interpreter
    - be mycology compliant for all fingerprints added
    - support all of the fingerprints listed [here](ww.rcfunge98.com/rcfunge2_manual.html)
- have a nice library interface for other applications
    - support extensions from applications in useful ways
- working debugger with nice interface
- support other dimensional funges than 2 (befunge)
    - mvrs fingerprint will require multiple amounts dimensions in different universes at the same time
- support running in a web browser via webassembly

