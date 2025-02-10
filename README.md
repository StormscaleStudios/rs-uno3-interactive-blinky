# Interactive Blinky (Rust on Arduino Uno R3)
This project is a personal starting point and learning sandbox for creating interactive embedded software in Rust.

The concept is simpel: A chain of LED lights that light up one-by-one, and a button to toggle the direction in which the lights are being cycled. See the [[Schematic subsection|#schematic]].

This project touches on quite a few fundamental learning steps:
- Compiling Rust projects for an AVR target
- Digital in- and output
- Timekeeping
- Interupt handling

There are a few variations of the project that are in the pipeline, for learning purposes. See the [[Roadmap subsection|#roadmap]].

## Schematic
-- todo: illustration/schematic of wiring

## Roadmap
- [ ] v0.0.1
    - hardware interactions through Hardware Abstraction Layer (HAL), provided by Rahix' ["avr-hal" project](https://github.com/Rahix/avr-hal).
    - core event loop based on polling tasks
- [ ] v0.0.2
    - hardware interactions based on Memory-mapped I/O (MMIO), effectively writing a custom HAL.
- [ ] v0.0.3
    - core event loop by implementing Futures 
- [ ] v0.0.4
    - core event loop by implementing async Rust

## Credits
The original implementation borrows heavily from the embedded Rust video series published by [The Rusty Bits](https://www.youtube.com/@therustybits) on Youtube.

Rahix's blogpost about [creating a Rust implementation for the ```millis()``` method](https://blog.rahix.de/005-avr-hal-millis/) available in the Arduino C libraries has been a invaluable resources for figuring out how to do interupt handling in Rust.

The contents of Rahix's ["avr-hal" project](https://github.com/Rahix/avr-hal) has been crucial part of getting a project that can run on the AVR architecture. 

## License
In order to comply with project dependencies, this repository adheres to the MIT license.

See [license file](LICENSE.md) or [original description](http://opensource.org/licenses/MIT).
