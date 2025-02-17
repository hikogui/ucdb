# Unicode Database (UCDB)
This is part of the HikoGUI project.

This crate contains the Unicode Database (UCDB) which will be build directly
into your application. It is a fast and memory efficient. It can be used for
various purposes such as normalization, text segmentation, script detection,
bidirectional language support, etc. 

Most of the database is used for looking attributes of a code-point, it does
this using a O(1) two-step associative lookup. The composition/decomposition
tables are a bit more complex.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

