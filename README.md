# LED Strip VM

A virtual machine for controlling a wifi led strip.

## The assemblerish language and bytecode

### Instructions

| assy                | meaning                                                                      | bc   |
| ------------------- | ---------------------------------------------------------------------------- | ---- |
| exit \<rg>          | exists the program with exit code \<rg>                                      | 0x01 |
| set \<byte> \<rg>   | sets the register rg to the static value \<byte>                             | 0x02 |
| copy \<rg> \<rg>    | copies the value of register a into register b                               | 0x03 |
| load (rgp, rgd)     | loads the value the pointer register points to into the data register        | 0x04 |
| clear \<rg>         | clears a register (sets it to 0x00)                                          | 0x05 |
| write (rgd, rgp)    | writes the value in the data register to the address of the pointer register | 0x06 |
| label <byte>        | creates a label at the current position.                                     | 0x07 |
| goto (rgl)          | goes to the label with name rgl                                              | 0x08 |
| debug (*)           | prints out the state for debug information                                   | 0x09 |
| add (rgd, rgi, rgo) | adds the value of rgi to the value of rgd and writes the result into rgo     | 0x10 |
| sub (rgd, rgi, rgo) | substracts rgi from rgd and writes the result into rgo                       | 0x11 |
| mul (rgd, rgi, rgo) | multiplies rgd by rgi and writes the result to rgo                           | 0x12 |
| div (rgd, rgi, rgo) | divides rgd by rgi and writes the result to rgo                              | 0x13 |
| mod (rgd, rgi, rgo) | applies mod rgd, rgi and writes the result to rgo                            | 0x14 |
| lsh (rgd, rgi, rgo) | bitshifts rgd by rgi to the left and writes the output to rgo                | 0x15 |
| rsh (rgd, rgi,rgo)  | bitshifts rgd by rgi to the right and writes the output to rgo               | 0x16 |
| jg (rgd, rgi, rgl)  | jumps to rgl if rgd > rgi                                                    | 0x20 |
| jl (rgd, rgi, rgl)  | jumps to rgl if rgd < rgi                                                    | 0x21 |
| je (rgd, rgi, rgl)  | jumps to rgl if rgd == rgi                                                   | 0x22 |
| pause (rgd)         | pauses for rgd milliseconds                                                  | 0xF0 |
| cmd (rgd)           | executes the command in rgd                                                  | 0xF1 |
| send (rcr, rcg, rcb)| sends the values stored in the color registers to the strip                  | 0xF2 |

### Registers

| register | type           | size    | number |
| -------- | -------------- | ------- | ------ |
| rcs      | state register | 1 bit   | 0x01   |
| rcr      | red value      | 1 byte  | 0x02   |
| rcg      | green value    | 1 byte  | 0x03   |
| rcb      | blue value     | 1 byte  | 0x04   |
| rgd      | data           | 4 bytes | 0x05   |
| rgp      | pointer        | 4 bytes | 0x06   |
| rgi      | input          | 4 bytes | 0x07   |
| rgo      | output         | 4 bytes | 0x08   |
| rgl      | label          | 4 bytes | 0x09   |

- changing the state register results in turning the strip on/off
- the rcr, rcg and rcb registers store the rgb value of the strip
- the rgd register is the default register for storing data
- the rgp register stores a pointer to "memory" space
- the rgi register stores the input for operations that require two input values
- the rgo register stores the result of operations
- the rgl register stores as label name that can be jumped to
- comments start with #

## The Runtime

The runtime works in three stages.

1. Connect to the led strip
2. Parse the bytecode into a vector of tokens
    - if the instruction creates a label, add the label to the map of labels
3. Execute the token vector