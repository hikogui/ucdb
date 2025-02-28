Contributing to the HikoGUI/ucdb project
========================================

Issues
------

Bugs and other issues can be reported on the [Issue](https://github.com/hikogui/ucdb/issues)
page on github.

The easiest way to contribute is by reporting issues with HikoGUI/ucdb.
When reporting an issue with HikoGUI/ucdb, make sure to clearly state:

 - The machine setup: "Windows 10, rustc 1.84.0."
 - The steps to reproduce: "I build using `cargo build`"
 - The outcome you expected: "I expected the build be successful"
 - The actual outcome: "I got an assertion at build\_src/parsers.rs:150"

Pull Requests
-------------

We are happy to accept pull requests for fixes, features and new tables.
In order to avoid wasting your time, we highly encourage opening an
[Discussion](https://github.com/hikogui/ucdb/discussions) or an
[Issue](https://github.com/hikogui/ucdb/issues) to discuss
whether the PR you're thinking about making will be acceptable.

If you like to work on an already existing [Issue](https://github.com/hikogui/ucdb/issues),
you may want to assign yourself to that issue before working on it,
to reduce the chance of two people working on the same pull request.

It could be helpful having a more real time discussion through discord at:
[Society of TJ](https://discord.gg/CSddDuM) channel #HikoGUI/general.

Installation and Build Instructions
-----------------------------------

To build ucdb:

```
cargo build
```

To fully test you should also test in `release` mode, certain tests are only enabled
in release mode as they are more exaustive and will take too slow in debug mode.

```
cargo test
cargo test --release
```

Code of Conduct
---------------

This project and everyone participating in it is governed by the
[HikoGUI Code of Conduct](CODE_OF_CONDUCT.md)

