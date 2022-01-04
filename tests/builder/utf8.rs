#![cfg(not(windows))]

use clap::{arg, App, AppSettings, Arg, ErrorKind};
use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;

#[test]
fn invalid_utf8_strict_positional() {
    let m = App::new("bad_utf8")
        .arg(Arg::new("arg"))
        .try_get_matches_from(vec![OsString::from(""), OsString::from_vec(vec![0xe9])]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidUtf8);
}

#[test]
fn invalid_utf8_strict_option_short_space() {
    let m = App::new("bad_utf8")
        .arg(Arg::new("arg").short('a').long("arg").takes_value(true))
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from("-a"),
            OsString::from_vec(vec![0xe9]),
        ]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidUtf8);
}

#[test]
fn invalid_utf8_strict_option_short_equals() {
    let m = App::new("bad_utf8")
        .arg(Arg::new("arg").short('a').long("arg").takes_value(true))
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from_vec(vec![0x2d, 0x61, 0x3d, 0xe9]),
        ]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidUtf8);
}

#[test]
fn invalid_utf8_strict_option_short_no_space() {
    let m = App::new("bad_utf8")
        .arg(Arg::new("arg").short('a').long("arg").takes_value(true))
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from_vec(vec![0x2d, 0x61, 0xe9]),
        ]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidUtf8);
}

#[test]
fn invalid_utf8_strict_option_long_space() {
    let m = App::new("bad_utf8")
        .arg(Arg::new("arg").short('a').long("arg").takes_value(true))
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from("--arg"),
            OsString::from_vec(vec![0xe9]),
        ]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidUtf8);
}

#[test]
fn invalid_utf8_strict_option_long_equals() {
    let m = App::new("bad_utf8")
        .arg(Arg::new("arg").short('a').long("arg").takes_value(true))
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from_vec(vec![0x2d, 0x2d, 0x61, 0x72, 0x67, 0x3d, 0xe9]),
        ]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidUtf8);
}

#[test]
fn invalid_utf8_lossy_positional() {
    let r = App::new("bad_utf8")
        .arg(Arg::new("arg").allow_invalid_utf8(true))
        .try_get_matches_from(vec![OsString::from(""), OsString::from_vec(vec![0xe9])]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(&*m.value_of_lossy("arg").unwrap(), "\u{FFFD}");
}

#[test]
fn invalid_utf8_lossy_option_short_space() {
    let r = App::new("bad_utf8")
        .arg(
            Arg::new("arg")
                .short('a')
                .long("arg")
                .takes_value(true)
                .allow_invalid_utf8(true),
        )
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from("-a"),
            OsString::from_vec(vec![0xe9]),
        ]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(&*m.value_of_lossy("arg").unwrap(), "\u{FFFD}");
}

#[test]
fn invalid_utf8_lossy_option_short_equals() {
    let r = App::new("bad_utf8")
        .arg(
            Arg::new("arg")
                .short('a')
                .long("arg")
                .takes_value(true)
                .allow_invalid_utf8(true),
        )
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from_vec(vec![0x2d, 0x61, 0x3d, 0xe9]),
        ]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(&*m.value_of_lossy("arg").unwrap(), "\u{FFFD}");
}

#[test]
fn invalid_utf8_lossy_option_short_no_space() {
    let r = App::new("bad_utf8")
        .arg(
            Arg::new("arg")
                .short('a')
                .long("arg")
                .takes_value(true)
                .allow_invalid_utf8(true),
        )
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from_vec(vec![0x2d, 0x61, 0xe9]),
        ]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(&*m.value_of_lossy("arg").unwrap(), "\u{FFFD}");
}

#[test]
fn invalid_utf8_lossy_option_long_space() {
    let r = App::new("bad_utf8")
        .arg(
            Arg::new("arg")
                .short('a')
                .long("arg")
                .takes_value(true)
                .allow_invalid_utf8(true),
        )
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from("--arg"),
            OsString::from_vec(vec![0xe9]),
        ]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(&*m.value_of_lossy("arg").unwrap(), "\u{FFFD}");
}

#[test]
fn invalid_utf8_lossy_option_long_equals() {
    let r = App::new("bad_utf8")
        .arg(
            Arg::new("arg")
                .short('a')
                .long("arg")
                .takes_value(true)
                .allow_invalid_utf8(true),
        )
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from_vec(vec![0x2d, 0x2d, 0x61, 0x72, 0x67, 0x3d, 0xe9]),
        ]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(&*m.value_of_lossy("arg").unwrap(), "\u{FFFD}");
}

#[test]
fn invalid_utf8_positional() {
    let r = App::new("bad_utf8")
        .arg(Arg::new("arg").allow_invalid_utf8(true))
        .try_get_matches_from(vec![OsString::from(""), OsString::from_vec(vec![0xe9])]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(
        &*m.value_of_os("arg").unwrap(),
        &*OsString::from_vec(vec![0xe9])
    );
}

#[test]
fn invalid_utf8_option_short_space() {
    let r = App::new("bad_utf8")
        .arg(
            Arg::new("arg")
                .short('a')
                .long("arg")
                .takes_value(true)
                .allow_invalid_utf8(true),
        )
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from("-a"),
            OsString::from_vec(vec![0xe9]),
        ]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(
        &*m.value_of_os("arg").unwrap(),
        &*OsString::from_vec(vec![0xe9])
    );
}

#[test]
fn invalid_utf8_option_short_equals() {
    let r = App::new("bad_utf8")
        .arg(
            Arg::new("arg")
                .short('a')
                .long("arg")
                .takes_value(true)
                .allow_invalid_utf8(true),
        )
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from_vec(vec![0x2d, 0x61, 0x3d, 0xe9]),
        ]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(
        &*m.value_of_os("arg").unwrap(),
        &*OsString::from_vec(vec![0xe9])
    );
}

#[test]
fn invalid_utf8_option_short_no_space() {
    let r = App::new("bad_utf8")
        .arg(
            Arg::new("arg")
                .short('a')
                .long("arg")
                .takes_value(true)
                .allow_invalid_utf8(true),
        )
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from_vec(vec![0x2d, 0x61, 0xe9]),
        ]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(
        &*m.value_of_os("arg").unwrap(),
        &*OsString::from_vec(vec![0xe9])
    );
}

#[test]
fn invalid_utf8_option_long_space() {
    let r = App::new("bad_utf8")
        .arg(
            Arg::new("arg")
                .short('a')
                .long("arg")
                .takes_value(true)
                .allow_invalid_utf8(true),
        )
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from("--arg"),
            OsString::from_vec(vec![0xe9]),
        ]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(
        &*m.value_of_os("arg").unwrap(),
        &*OsString::from_vec(vec![0xe9])
    );
}

#[test]
fn invalid_utf8_option_long_equals() {
    let r = App::new("bad_utf8")
        .arg(
            Arg::new("arg")
                .short('a')
                .long("arg")
                .takes_value(true)
                .allow_invalid_utf8(true),
        )
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from_vec(vec![0x2d, 0x2d, 0x61, 0x72, 0x67, 0x3d, 0xe9]),
        ]);
    assert!(r.is_ok(), "{}", r.unwrap_err());
    let m = r.unwrap();
    assert!(m.is_present("arg"));
    assert_eq!(
        &*m.value_of_os("arg").unwrap(),
        &*OsString::from_vec(vec![0xe9])
    );
}

#[test]
fn refuse_invalid_utf8_subcommand_with_allow_external_subcommands() {
    let m = App::new("bad_utf8")
        .setting(AppSettings::AllowExternalSubcommands)
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from_vec(vec![0xe9]),
            OsString::from("normal"),
        ]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidUtf8);
}

#[test]
fn refuse_invalid_utf8_subcommand_when_args_are_allowed_with_allow_external_subcommands() {
    let m = App::new("bad_utf8")
        .setting(AppSettings::AllowExternalSubcommands)
        .setting(AppSettings::AllowInvalidUtf8ForExternalSubcommands)
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from_vec(vec![0xe9]),
            OsString::from("normal"),
        ]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidUtf8);
}

#[test]
fn refuse_invalid_utf8_subcommand_args_with_allow_external_subcommands() {
    let m = App::new("bad_utf8")
        .setting(AppSettings::AllowExternalSubcommands)
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from("subcommand"),
            OsString::from("normal"),
            OsString::from_vec(vec![0xe9]),
            OsString::from("--another_normal"),
        ]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::InvalidUtf8);
}

#[test]
fn allow_invalid_utf8_subcommand_args_with_allow_external_subcommands() {
    let m = App::new("bad_utf8")
        .setting(AppSettings::AllowExternalSubcommands)
        .setting(AppSettings::AllowInvalidUtf8ForExternalSubcommands)
        .try_get_matches_from(vec![
            OsString::from(""),
            OsString::from("subcommand"),
            OsString::from("normal"),
            OsString::from_vec(vec![0xe9]),
            OsString::from("--another_normal"),
        ]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();
    let (subcommand, args) = m.subcommand().unwrap();
    let args = args.values_of_os("").unwrap().collect::<Vec<_>>();
    assert_eq!(subcommand, OsString::from("subcommand"));
    assert_eq!(
        args,
        vec![
            OsString::from("normal"),
            OsString::from_vec(vec![0xe9]),
            OsString::from("--another_normal"),
        ]
    );
}

#[test]
fn allow_validated_utf8_value_of() {
    let a = App::new("test").arg(arg!(--name <NAME>));
    let m = a.try_get_matches_from(["test", "--name", "me"]).unwrap();
    let _ = m.value_of("name");
}

#[test]
#[should_panic = "Must use `Arg::allow_invalid_utf8` with `_os` lookups at `name`"]
fn panic_validated_utf8_value_of_os() {
    let a = App::new("test").arg(arg!(--name <NAME>));
    let m = a.try_get_matches_from(["test", "--name", "me"]).unwrap();
    let _ = m.value_of_os("name");
}

#[test]
fn ignore_validated_utf8_with_defaults() {
    // For now, we don't check the correct call is used with defaults (due to pain of piping it
    // through the code) but we need to make sure we don't erroneously panic.
    let a = App::new("test").arg(arg!(--value <VALUE>).required(false).default_value("foo"));
    let m = a.try_get_matches_from(["test"]).unwrap();
    let _ = m.value_of("value");
    let _ = m.value_of_os("value");
}

#[test]
fn allow_invalid_utf8_value_of_os() {
    let a = App::new("test").arg(arg!(--name <NAME>).allow_invalid_utf8(true));
    let m = a.try_get_matches_from(["test", "--name", "me"]).unwrap();
    let _ = m.value_of_os("name");
}

#[test]
#[should_panic = "Must use `_os` lookups with `Arg::allow_invalid_utf8` at `name`"]
fn panic_invalid_utf8_value_of() {
    let a = App::new("test").arg(arg!(--name <NAME>).allow_invalid_utf8(true));
    let m = a.try_get_matches_from(["test", "--name", "me"]).unwrap();
    let _ = m.value_of("name");
}

#[test]
fn ignore_invalid_utf8_with_defaults() {
    // For now, we don't check the correct call is used with defaults (due to pain of piping it
    // through the code) but we need to make sure we don't erroneously panic.
    let a = App::new("test").arg(
        arg!(--value <VALUE>)
            .required(false)
            .default_value("foo")
            .allow_invalid_utf8(true),
    );
    let m = a.try_get_matches_from(["test"]).unwrap();
    let _ = m.value_of("value");
    let _ = m.value_of_os("value");
}

#[test]
fn allow_validated_utf8_external_subcommand_values_of() {
    let a = App::new("test").setting(AppSettings::AllowExternalSubcommands);
    let m = a.try_get_matches_from(vec!["test", "cmd", "arg"]).unwrap();
    let (_ext, args) = m.subcommand().unwrap();
    let _ = args.values_of("");
}

#[test]
#[should_panic = "Must use `Arg::allow_invalid_utf8` with `_os` lookups at ``"]
fn panic_validated_utf8_external_subcommand_values_of_os() {
    let a = App::new("test").setting(AppSettings::AllowExternalSubcommands);
    let m = a.try_get_matches_from(vec!["test", "cmd", "arg"]).unwrap();
    let (_ext, args) = m.subcommand().unwrap();
    let _ = args.values_of_os("");
}

#[test]
fn allow_invalid_utf8_external_subcommand_values_of_os() {
    let a = App::new("test")
        .setting(AppSettings::AllowExternalSubcommands)
        .setting(AppSettings::AllowInvalidUtf8ForExternalSubcommands);
    let m = a.try_get_matches_from(vec!["test", "cmd", "arg"]).unwrap();
    let (_ext, args) = m.subcommand().unwrap();
    let _ = args.values_of_os("");
}

#[test]
#[should_panic = "Must use `_os` lookups with `Arg::allow_invalid_utf8` at ``"]
fn panic_invalid_utf8_external_subcommand_values_of() {
    let a = App::new("test")
        .setting(AppSettings::AllowExternalSubcommands)
        .setting(AppSettings::AllowInvalidUtf8ForExternalSubcommands);
    let m = a.try_get_matches_from(vec!["test", "cmd", "arg"]).unwrap();
    let (_ext, args) = m.subcommand().unwrap();
    let _ = args.values_of("");
}
