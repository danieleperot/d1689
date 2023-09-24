# D1689 CPU emulator

D1689 is not a real CPU / instruction set. It's a very simple CPU I made to help
me learn how to program applications in an assembly language.

## Quick start
```shell
cargo run --release
```

## Architecture
-- Coming soon! --

Registers:
* `A`: general purpose, 8 bit
* `B`: general purpose, 8 bit
* `Flags`: `0b??????ZC`
  * `C`: Carry flag. Set to `true` when the previous operation caused overflow.
  * `Z`: Zero flag. Set to `true` when the previous operation returned zero.

No memory / program counter / stack yet!

## Instructions set
-- Coming soon! --

Please note: there isn't a way yet to laod instructions from file. Instead, the
instructions have to be manually executed one by one.

### Load operations
`LOAD [destination register], [8 bit value]`
* does not change flags

### Acccumulator
`ADD [destination register], [8 bit value]`
* sets `Z` if the result was zero, sets `C` if the result was over `0xFF` (overflow)
`ADD [destination register], [source register]`
* sets `Z` if the result was zero, sets `C` if the result was over `0xFF` (overflow)

