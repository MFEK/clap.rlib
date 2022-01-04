1. [Comparisons](#comparisons)
   1. [How does `clap` compare to structopt?](#how-does-clap-compare-to-structopt)
   2. [How does `clap` compare to getopts?](#how-does-clap-compare-to-getopts)
   3. [How does `clap` compare to docopt.rs?](#how-does-clap-compare-to-docoptrs)
   4. [What are some reasons to use `clap`? (The Pitch)](#what-are-some-reasons-to-use-clap-the-pitch)
   5. [What are some reasons *not* to use `clap`? (The Anti Pitch)](#what-are-some-reasons-not-to-use-clap-the-anti-pitch)
   6. [Reasons to use `clap`](#reasons-to-use-clap)
   7. [Reasons to `docopt`](#reasons-to-docopt)
   8. [Reasons to use `getopts`](#reasons-to-use-getopts)
2. [How many methods are there to create an App/Arg?](#how-many-methods-are-there-to-create-an-apparg)
3. [Why is there a default subcommand of help?](#why-is-there-a-default-subcommand-of-help)

### Comparisons

First, let me say that these comparisons are highly subjective, and not meant
in a critical or harsh manner. All the argument parsing libraries out there (to
include `clap`) have their own strengths and weaknesses. Sometimes it just
comes down to personal taste when all other factors are equal. When in doubt,
try them all and pick one that you enjoy :) There's plenty of room in the Rust
community for multiple implementations!

For less detailed but more broad comparisons, see
[argparse-benchmarks](https://github.com/rust-cli/argparse-benchmarks-rs).

#### How does `clap` compare to [structopt](https://github.com/TeXitoi/structopt)?

Simple! `clap` *is* `structopt`.  `structopt` started as a derive API built on
top of clap v2.  With clap v3, we've forked structopt and integrated it
directly into clap.  structopt is in
[maintenance mode](https://github.com/TeXitoi/structopt/issues/516#issuecomment-989566094)
with the release of `clap_derive`.

The benefits of integrating `structopt` and `clap` are:
- Easier cross-linking in documentation
- [Documentation parity](../examples)
- Tighter design feedback loop, ensuring all new features are designed with
  derives in mind and easier to change `clap` in response to `structopt` bugs.
- Clearer endorsement of `structopt`

For more details on what has changed and how to migrate, see the [CHANGELOG](../CHANGELOG.md)

#### What are some reasons to use `clap`? (The Pitch)

`clap` is as fast, and as lightweight as possible while still giving all the features you'd expect from a modern argument parser. In fact, for the amount and type of features `clap` offers it remains about as fast as `getopts`. If you use `clap` when just need some simple arguments parsed, you'll find it's a walk in the park. `clap` also makes it possible to represent extremely complex, and advanced requirements, without too much thought. `clap` aims to be intuitive, easy to use, and fully capable for wide variety use cases and needs.

#### What are some reasons *not* to use `clap`? (The Anti Pitch)

Depending on the style in which you choose to define the valid arguments, `clap` can be very verbose. `clap` also offers so many finetuning knobs and dials, that learning everything can seem overwhelming. I strive to keep the simple cases simple, but when turning all those custom dials it can get complex. `clap` is also opinionated about parsing. Even though so much can be tweaked and tuned with `clap` (and I'm adding more all the time), there are still certain features which `clap` implements in specific ways which may be contrary to some users use-cases.

#### Reasons to use `clap`

 * You want all the nice CLI features your users may expect, yet you don't want to implement them all yourself. You'd like to focus your application, not argument parsing.
 * In addition to the point above; you don't want to sacrifice performance to get all those nice features
 * You have complex requirements/conflicts between your various valid args.
 * You want to use subcommands (although other libraries also support subcommands, they are not nearly as feature rich as those provided by `clap`)
 * You want some sort of custom validation built into the argument parsing process, instead of as part of your application (which allows for earlier failures, better error messages, more cohesive experience, etc.)

### How many approaches are there to create a parser?

The following APIs are supported:
- [Derive](../examples/tutorial_derive/README.md)
- [Builder](../examples/tutorial_builder/README.md)

Previously, we supported:
- [YAML](https://github.com/clap-rs/clap/issues/3087)
- [docopt](http://docopt.org/)-inspired [usage parser](https://github.com/clap-rs/clap/issues/3086)
- [`clap_app!`](https://github.com/clap-rs/clap/issues/2835)

There are also experiments with other APIs:
- [fncmd](https://github.com/yuhr/fncmd): function attribute

### Why is there a default subcommand of help?

There is only a default subcommand of `help` when other subcommands have been defined manually. So it's opt-in(ish), being that you only get a `help` subcommand if you're actually using subcommands.

Also, if the user defined a `help` subcommand themselves, the auto-generated one wouldn't be added (meaning it's only generated if the user hasn't defined one themselves).

