use crate::utils;

use clap::{arg, App, Arg, ArgGroup, ErrorKind};

static REQUIRE_EQUALS: &str = "error: The following required arguments were not provided:
    --opt=<FILE>

USAGE:
    clap-test --opt=<FILE>

For more information try --help
";

static REQUIRE_EQUALS_FILTERED: &str = "error: The following required arguments were not provided:
    --opt=<FILE>

USAGE:
    clap-test --opt=<FILE> --foo=<FILE>

For more information try --help
";

static REQUIRE_EQUALS_FILTERED_GROUP: &str =
    "error: The following required arguments were not provided:
    --opt=<FILE>

USAGE:
    clap-test --opt=<FILE> --foo=<FILE> <--g1=<FILE>|--g2=<FILE>>

For more information try --help
";

static MISSING_REQ: &str = "error: The following required arguments were not provided:
    --long-option-2 <option2>
    <positional2>

USAGE:
    clap-test --long-option-2 <option2> -F <positional2>

For more information try --help
";

static COND_REQ_IN_USAGE: &str = "error: The following required arguments were not provided:
    --output <output>

USAGE:
    test --target <target> --input <input> --output <output>

For more information try --help
";

#[test]
fn flag_required() {
    let result = App::new("flag_required")
        .arg(arg!(-f --flag "some flag").requires("color"))
        .arg(arg!(-c --color "third flag"))
        .try_get_matches_from(vec!["", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn flag_required_2() {
    let m = App::new("flag_required")
        .arg(arg!(-f --flag "some flag").requires("color"))
        .arg(arg!(-c --color "third flag"))
        .try_get_matches_from(vec!["", "-f", "-c"])
        .unwrap();
    assert!(m.is_present("color"));
    assert!(m.is_present("flag"));
}

#[test]
fn option_required() {
    let result = App::new("option_required")
        .arg(arg!(f: -f <flag> "some flag").required(false).requires("c"))
        .arg(arg!(c: -c <color> "third flag").required(false))
        .try_get_matches_from(vec!["", "-f", "val"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn option_required_2() {
    let m = App::new("option_required")
        .arg(arg!(f: -f <flag> "some flag").required(false).requires("c"))
        .arg(arg!(c: -c <color> "third flag").required(false))
        .try_get_matches_from(vec!["", "-f", "val", "-c", "other_val"])
        .unwrap();
    assert!(m.is_present("c"));
    assert_eq!(m.value_of("c").unwrap(), "other_val");
    assert!(m.is_present("f"));
    assert_eq!(m.value_of("f").unwrap(), "val");
}

#[test]
fn positional_required() {
    let result = App::new("positional_required")
        .arg(Arg::new("flag").index(1).required(true))
        .try_get_matches_from(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn positional_required_2() {
    let m = App::new("positional_required")
        .arg(Arg::new("flag").index(1).required(true))
        .try_get_matches_from(vec!["", "someval"])
        .unwrap();
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "someval");
}

#[test]
fn group_required() {
    let result = App::new("group_required")
        .arg(arg!(-f --flag "some flag"))
        .group(ArgGroup::new("gr").required(true).arg("some").arg("other"))
        .arg(arg!(--some "some arg"))
        .arg(arg!(--other "other arg"))
        .try_get_matches_from(vec!["", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn group_required_2() {
    let m = App::new("group_required")
        .arg(arg!(-f --flag "some flag"))
        .group(ArgGroup::new("gr").required(true).arg("some").arg("other"))
        .arg(arg!(--some "some arg"))
        .arg(arg!(--other "other arg"))
        .try_get_matches_from(vec!["", "-f", "--some"])
        .unwrap();
    assert!(m.is_present("some"));
    assert!(!m.is_present("other"));
    assert!(m.is_present("flag"));
}

#[test]
fn group_required_3() {
    let m = App::new("group_required")
        .arg(arg!(-f --flag "some flag"))
        .group(ArgGroup::new("gr").required(true).arg("some").arg("other"))
        .arg(arg!(--some "some arg"))
        .arg(arg!(--other "other arg"))
        .try_get_matches_from(vec!["", "-f", "--other"])
        .unwrap();
    assert!(!m.is_present("some"));
    assert!(m.is_present("other"));
    assert!(m.is_present("flag"));
}

#[test]
fn arg_require_group() {
    let result = App::new("arg_require_group")
        .arg(arg!(-f --flag "some flag").requires("gr"))
        .group(ArgGroup::new("gr").arg("some").arg("other"))
        .arg(arg!(--some "some arg"))
        .arg(arg!(--other "other arg"))
        .try_get_matches_from(vec!["", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn arg_require_group_2() {
    let res = App::new("arg_require_group")
        .arg(arg!(-f --flag "some flag").requires("gr"))
        .group(ArgGroup::new("gr").arg("some").arg("other"))
        .arg(arg!(--some "some arg"))
        .arg(arg!(--other "other arg"))
        .try_get_matches_from(vec!["", "-f", "--some"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(m.is_present("some"));
    assert!(!m.is_present("other"));
    assert!(m.is_present("flag"));
}

#[test]
fn arg_require_group_3() {
    let res = App::new("arg_require_group")
        .arg(arg!(-f --flag "some flag").requires("gr"))
        .group(ArgGroup::new("gr").arg("some").arg("other"))
        .arg(arg!(--some "some arg"))
        .arg(arg!(--other "other arg"))
        .try_get_matches_from(vec!["", "-f", "--other"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(!m.is_present("some"));
    assert!(m.is_present("other"));
    assert!(m.is_present("flag"));
}

// REQUIRED_UNLESS

#[test]
fn issue_753() {
    let m = App::new("test")
        .arg(arg!(
            -l --list "List available interfaces (and stop there)"
        ))
        .arg(
            arg!(
                -i --iface <INTERFACE> "Ethernet interface for fetching NTP packets"
            )
            .required(false)
            .required_unless_present("list"),
        )
        .arg(
            arg!(-f --file <TESTFILE> "Fetch NTP packets from pcap file")
                .required(false)
                .conflicts_with("iface")
                .required_unless_present("list"),
        )
        .arg(
            arg!(-s --server <SERVER_IP> "NTP server IP address")
                .required(false)
                .required_unless_present("list"),
        )
        .try_get_matches_from(vec!["test", "--list"]);
    assert!(m.is_ok(), "{}", m.unwrap_err());
}

#[test]
fn required_unless_present() {
    let res = App::new("unlesstest")
        .arg(
            Arg::new("cfg")
                .required_unless_present("dbg")
                .takes_value(true)
                .long("config"),
        )
        .arg(Arg::new("dbg").long("debug"))
        .try_get_matches_from(vec!["unlesstest", "--debug"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(m.is_present("dbg"));
    assert!(!m.is_present("cfg"));
}

#[test]
fn required_unless_err() {
    let res = App::new("unlesstest")
        .arg(
            Arg::new("cfg")
                .required_unless_present("dbg")
                .takes_value(true)
                .long("config"),
        )
        .arg(Arg::new("dbg").long("debug"))
        .try_get_matches_from(vec!["unlesstest"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn required_unless_present_with_optional_value() {
    let res = App::new("unlesstest")
        .arg(Arg::new("opt").long("opt").min_values(0).max_values(1))
        .arg(
            Arg::new("cfg")
                .required_unless_present("dbg")
                .takes_value(true)
                .long("config"),
        )
        .arg(Arg::new("dbg").long("debug"))
        .try_get_matches_from(vec!["unlesstest", "--opt"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
}

// REQUIRED_UNLESS_ALL

#[test]
fn required_unless_present_all() {
    let res = App::new("unlessall")
        .arg(
            Arg::new("cfg")
                .required_unless_present_all(&["dbg", "infile"])
                .takes_value(true)
                .long("config"),
        )
        .arg(Arg::new("dbg").long("debug"))
        .arg(Arg::new("infile").short('i').takes_value(true))
        .try_get_matches_from(vec!["unlessall", "--debug", "-i", "file"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(m.is_present("dbg"));
    assert!(m.is_present("infile"));
    assert!(!m.is_present("cfg"));
}

#[test]
fn required_unless_all_err() {
    let res = App::new("unlessall")
        .arg(
            Arg::new("cfg")
                .required_unless_present_all(&["dbg", "infile"])
                .takes_value(true)
                .long("config"),
        )
        .arg(Arg::new("dbg").long("debug"))
        .arg(Arg::new("infile").short('i').takes_value(true))
        .try_get_matches_from(vec!["unlessall", "--debug"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
}

// REQUIRED_UNLESS_ONE

#[test]
fn required_unless_present_any() {
    let res = App::new("unlessone")
        .arg(
            Arg::new("cfg")
                .required_unless_present_any(&["dbg", "infile"])
                .takes_value(true)
                .long("config"),
        )
        .arg(Arg::new("dbg").long("debug"))
        .arg(Arg::new("infile").short('i').takes_value(true))
        .try_get_matches_from(vec!["unlessone", "--debug"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(m.is_present("dbg"));
    assert!(!m.is_present("cfg"));
}

#[test]
fn required_unless_any_2() {
    // This tests that the required_unless_present_any works when the second arg in the array is used
    // instead of the first.
    let res = App::new("unlessone")
        .arg(
            Arg::new("cfg")
                .required_unless_present_any(&["dbg", "infile"])
                .takes_value(true)
                .long("config"),
        )
        .arg(Arg::new("dbg").long("debug"))
        .arg(Arg::new("infile").short('i').takes_value(true))
        .try_get_matches_from(vec!["unlessone", "-i", "file"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(m.is_present("infile"));
    assert!(!m.is_present("cfg"));
}

#[test]
fn required_unless_any_works_with_short() {
    // GitHub issue: https://github.com/clap-rs/clap/issues/1135
    let res = App::new("unlessone")
        .arg(Arg::new("a").conflicts_with("b").short('a'))
        .arg(Arg::new("b").short('b'))
        .arg(
            Arg::new("x")
                .short('x')
                .required_unless_present_any(&["a", "b"]),
        )
        .try_get_matches_from(vec!["unlessone", "-a"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn required_unless_any_works_with_short_err() {
    let res = App::new("unlessone")
        .arg(Arg::new("a").conflicts_with("b").short('a'))
        .arg(Arg::new("b").short('b'))
        .arg(
            Arg::new("x")
                .short('x')
                .required_unless_present_any(&["a", "b"]),
        )
        .try_get_matches_from(vec!["unlessone"]);

    assert!(res.is_err());
}

#[test]
fn required_unless_any_works_without() {
    let res = App::new("unlessone")
        .arg(Arg::new("a").conflicts_with("b").short('a'))
        .arg(Arg::new("b").short('b'))
        .arg(Arg::new("x").required_unless_present_any(&["a", "b"]))
        .try_get_matches_from(vec!["unlessone", "-a"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn required_unless_any_works_with_long() {
    let res = App::new("unlessone")
        .arg(Arg::new("a").conflicts_with("b").short('a'))
        .arg(Arg::new("b").short('b'))
        .arg(
            Arg::new("x")
                .long("x_is_the_option")
                .required_unless_present_any(&["a", "b"]),
        )
        .try_get_matches_from(vec!["unlessone", "-a"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn required_unless_any_1() {
    let res = App::new("unlessone")
        .arg(
            Arg::new("cfg")
                .required_unless_present_any(&["dbg", "infile"])
                .takes_value(true)
                .long("config"),
        )
        .arg(Arg::new("dbg").long("debug"))
        .arg(Arg::new("infile").short('i').takes_value(true))
        .try_get_matches_from(vec!["unlessone", "--debug"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
    let m = res.unwrap();
    assert!(!m.is_present("infile"));
    assert!(!m.is_present("cfg"));
    assert!(m.is_present("dbg"));
}

#[test]
fn required_unless_any_err() {
    let res = App::new("unlessone")
        .arg(
            Arg::new("cfg")
                .required_unless_present_any(&["dbg", "infile"])
                .takes_value(true)
                .long("config"),
        )
        .arg(Arg::new("dbg").long("debug"))
        .arg(Arg::new("infile").short('i').takes_value(true))
        .try_get_matches_from(vec!["unlessone"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn missing_required_output() {
    assert!(utils::compare_output(
        utils::complex_app(),
        "clap-test -F",
        MISSING_REQ,
        true
    ));
}

// Conditional external requirements

#[test]
fn requires_if_present_val() {
    let res = App::new("unlessone")
        .arg(
            Arg::new("cfg")
                .requires_if("my.cfg", "extra")
                .takes_value(true)
                .long("config"),
        )
        .arg(Arg::new("extra").long("extra"))
        .try_get_matches_from(vec!["unlessone", "--config=my.cfg"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn requires_if_present_mult() {
    let res = App::new("unlessone")
        .arg(
            Arg::new("cfg")
                .requires_ifs(&[("my.cfg", "extra"), ("other.cfg", "other")])
                .takes_value(true)
                .long("config"),
        )
        .arg(Arg::new("extra").long("extra"))
        .arg(Arg::new("other").long("other"))
        .try_get_matches_from(vec!["unlessone", "--config=other.cfg"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn requires_if_present_mult_pass() {
    let res = App::new("unlessone")
        .arg(
            Arg::new("cfg")
                .requires_ifs(&[("my.cfg", "extra"), ("other.cfg", "other")])
                .takes_value(true)
                .long("config"),
        )
        .arg(Arg::new("extra").long("extra"))
        .arg(Arg::new("other").long("other"))
        .try_get_matches_from(vec!["unlessone", "--config=some.cfg"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn requires_if_present_val_no_present_pass() {
    let res = App::new("unlessone")
        .arg(
            Arg::new("cfg")
                .requires_if("my.cfg", "extra")
                .takes_value(true)
                .long("config"),
        )
        .arg(Arg::new("extra").long("extra"))
        .try_get_matches_from(vec!["unlessone"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
}

// Conditionally required

#[test]
fn required_if_val_present_pass() {
    let res = App::new("ri")
        .arg(
            Arg::new("cfg")
                .required_if_eq("extra", "val")
                .takes_value(true)
                .long("config"),
        )
        .arg(Arg::new("extra").takes_value(true).long("extra"))
        .try_get_matches_from(vec!["ri", "--extra", "val", "--config", "my.cfg"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn required_if_val_present_fail() {
    let res = App::new("ri")
        .arg(
            Arg::new("cfg")
                .required_if_eq("extra", "val")
                .takes_value(true)
                .long("config"),
        )
        .arg(Arg::new("extra").takes_value(true).long("extra"))
        .try_get_matches_from(vec!["ri", "--extra", "val"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn required_if_val_present_ignore_case_pass() {
    let res = App::new("ri")
        .arg(
            Arg::new("cfg")
                .required_if_eq("extra", "Val")
                .takes_value(true)
                .long("config"),
        )
        .arg(
            Arg::new("extra")
                .takes_value(true)
                .long("extra")
                .ignore_case(true),
        )
        .try_get_matches_from(vec!["ri", "--extra", "vaL", "--config", "my.cfg"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn required_if_val_present_ignore_case_fail() {
    let res = App::new("ri")
        .arg(
            Arg::new("cfg")
                .required_if_eq("extra", "Val")
                .takes_value(true)
                .long("config"),
        )
        .arg(
            Arg::new("extra")
                .takes_value(true)
                .long("extra")
                .ignore_case(true),
        )
        .try_get_matches_from(vec!["ri", "--extra", "vaL"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn required_if_all_values_present_pass() {
    let res = App::new("ri")
        .arg(
            Arg::new("cfg")
                .required_if_eq_all(&[("extra", "val"), ("option", "spec")])
                .takes_value(true)
                .long("config"),
        )
        .arg(Arg::new("extra").takes_value(true).long("extra"))
        .arg(Arg::new("option").takes_value(true).long("option"))
        .try_get_matches_from(vec![
            "ri", "--extra", "val", "--option", "spec", "--config", "my.cfg",
        ]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn required_if_some_values_present_pass() {
    let res = App::new("ri")
        .arg(
            Arg::new("cfg")
                .required_if_eq_all(&[("extra", "val"), ("option", "spec")])
                .takes_value(true)
                .long("config"),
        )
        .arg(Arg::new("extra").takes_value(true).long("extra"))
        .arg(Arg::new("option").takes_value(true).long("option"))
        .try_get_matches_from(vec!["ri", "--extra", "val"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn required_if_all_values_present_fail() {
    let res = App::new("ri")
        .arg(
            Arg::new("cfg")
                .required_if_eq_all(&[("extra", "val"), ("option", "spec")])
                .takes_value(true)
                .long("config"),
        )
        .arg(Arg::new("extra").takes_value(true).long("extra"))
        .arg(Arg::new("option").takes_value(true).long("option"))
        .try_get_matches_from(vec!["ri", "--extra", "val", "--option", "spec"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn required_if_any_all_values_present_pass() {
    let res = App::new("ri")
        .arg(
            Arg::new("cfg")
                .required_if_eq_all(&[("extra", "val"), ("option", "spec")])
                .required_if_eq_any(&[("extra", "val2"), ("option", "spec2")])
                .takes_value(true)
                .long("config"),
        )
        .arg(Arg::new("extra").takes_value(true).long("extra"))
        .arg(Arg::new("option").takes_value(true).long("option"))
        .try_get_matches_from(vec![
            "ri", "--extra", "val", "--option", "spec", "--config", "my.cfg",
        ]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn required_if_any_all_values_present_fail() {
    let res = App::new("ri")
        .arg(
            Arg::new("cfg")
                .required_if_eq_all(&[("extra", "val"), ("option", "spec")])
                .required_if_eq_any(&[("extra", "val2"), ("option", "spec2")])
                .takes_value(true)
                .long("config"),
        )
        .arg(Arg::new("extra").takes_value(true).long("extra"))
        .arg(Arg::new("option").takes_value(true).long("option"))
        .try_get_matches_from(vec!["ri", "--extra", "val", "--option", "spec"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn list_correct_required_args() {
    let app = App::new("Test app")
        .version("1.0")
        .author("F0x06")
        .about("Arg test")
        .arg(
            Arg::new("target")
                .takes_value(true)
                .required(true)
                .possible_values(["file", "stdout"])
                .long("target"),
        )
        .arg(
            Arg::new("input")
                .takes_value(true)
                .required(true)
                .long("input"),
        )
        .arg(
            Arg::new("output")
                .takes_value(true)
                .required(true)
                .long("output"),
        );

    assert!(utils::compare_output(
        app,
        "test --input somepath --target file",
        COND_REQ_IN_USAGE,
        true
    ));
}

#[test]
fn required_if_val_present_fail_error_output() {
    let app = App::new("Test app")
        .version("1.0")
        .author("F0x06")
        .about("Arg test")
        .arg(
            Arg::new("target")
                .takes_value(true)
                .required(true)
                .possible_values(&["file", "stdout"])
                .long("target"),
        )
        .arg(
            Arg::new("input")
                .takes_value(true)
                .required(true)
                .long("input"),
        )
        .arg(
            Arg::new("output")
                .takes_value(true)
                .required_if_eq("target", "file")
                .long("output"),
        );

    assert!(utils::compare_output(
        app,
        "test --input somepath --target file",
        COND_REQ_IN_USAGE,
        true
    ));
}

#[test]
fn required_if_wrong_val() {
    let res = App::new("ri")
        .arg(
            Arg::new("cfg")
                .required_if_eq("extra", "val")
                .takes_value(true)
                .long("config"),
        )
        .arg(Arg::new("extra").takes_value(true).long("extra"))
        .try_get_matches_from(vec!["ri", "--extra", "other"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn required_ifs_val_present_pass() {
    let res = App::new("ri")
        .arg(
            Arg::new("cfg")
                .required_if_eq_any(&[("extra", "val"), ("option", "spec")])
                .takes_value(true)
                .long("config"),
        )
        .arg(Arg::new("option").takes_value(true).long("option"))
        .arg(Arg::new("extra").takes_value(true).long("extra"))
        .try_get_matches_from(vec!["ri", "--option", "spec", "--config", "my.cfg"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn required_ifs_val_present_fail() {
    let res = App::new("ri")
        .arg(
            Arg::new("cfg")
                .required_if_eq_any(&[("extra", "val"), ("option", "spec")])
                .takes_value(true)
                .long("config"),
        )
        .arg(Arg::new("extra").takes_value(true).long("extra"))
        .arg(Arg::new("option").takes_value(true).long("option"))
        .try_get_matches_from(vec!["ri", "--option", "spec"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn required_ifs_wrong_val() {
    let res = App::new("ri")
        .arg(
            Arg::new("cfg")
                .required_if_eq_any(&[("extra", "val"), ("option", "spec")])
                .takes_value(true)
                .long("config"),
        )
        .arg(Arg::new("extra").takes_value(true).long("extra"))
        .arg(Arg::new("option").takes_value(true).long("option"))
        .try_get_matches_from(vec!["ri", "--option", "other"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn required_ifs_wrong_val_mult_fail() {
    let res = App::new("ri")
        .arg(
            Arg::new("cfg")
                .required_if_eq_any(&[("extra", "val"), ("option", "spec")])
                .takes_value(true)
                .long("config"),
        )
        .arg(Arg::new("extra").takes_value(true).long("extra"))
        .arg(Arg::new("option").takes_value(true).long("option"))
        .try_get_matches_from(vec!["ri", "--extra", "other", "--option", "spec"]);

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn require_eq() {
    let app = App::new("clap-test").version("v1.4.8").arg(
        Arg::new("opt")
            .long("opt")
            .short('o')
            .required(true)
            .require_equals(true)
            .value_name("FILE")
            .help("some"),
    );
    assert!(utils::compare_output(
        app,
        "clap-test",
        REQUIRE_EQUALS,
        true
    ));
}

#[test]
fn require_eq_filtered() {
    let app = App::new("clap-test")
        .version("v1.4.8")
        .arg(
            Arg::new("opt")
                .long("opt")
                .short('o')
                .required(true)
                .require_equals(true)
                .value_name("FILE")
                .help("some"),
        )
        .arg(
            Arg::new("foo")
                .long("foo")
                .short('f')
                .required(true)
                .require_equals(true)
                .value_name("FILE")
                .help("some other arg"),
        );
    assert!(utils::compare_output(
        app,
        "clap-test -f=blah",
        REQUIRE_EQUALS_FILTERED,
        true
    ));
}

#[test]
fn require_eq_filtered_group() {
    let app = App::new("clap-test")
        .version("v1.4.8")
        .arg(
            Arg::new("opt")
                .long("opt")
                .short('o')
                .required(true)
                .require_equals(true)
                .value_name("FILE")
                .help("some"),
        )
        .arg(
            Arg::new("foo")
                .long("foo")
                .short('f')
                .required(true)
                .require_equals(true)
                .value_name("FILE")
                .help("some other arg"),
        )
        .arg(
            Arg::new("g1")
                .long("g1")
                .require_equals(true)
                .value_name("FILE"),
        )
        .arg(
            Arg::new("g2")
                .long("g2")
                .require_equals(true)
                .value_name("FILE"),
        )
        .group(
            ArgGroup::new("test_group")
                .args(&["g1", "g2"])
                .required(true),
        );
    assert!(utils::compare_output(
        app,
        "clap-test -f=blah --g1=blah",
        REQUIRE_EQUALS_FILTERED_GROUP,
        true
    ));
}

static ISSUE_1158: &str = "error: The following required arguments were not provided:
    -x <X>
    -y <Y>
    -z <Z>

USAGE:
    example -x <X> -y <Y> -z <Z> <ID>

For more information try --help
";

fn issue_1158_app() -> App<'static> {
    App::new("example")
        .arg(
            arg!(-c --config <FILE> "Custom config file.")
                .required(false)
                .required_unless_present("ID")
                .conflicts_with("ID"),
        )
        .arg(
            arg!([ID] "ID")
                .required_unless_present("config")
                .conflicts_with("config")
                .requires_all(&["x", "y", "z"]),
        )
        .arg(arg!(x: -x <X> "X").required(false))
        .arg(arg!(y: -y <Y> "Y").required(false))
        .arg(arg!(z: -z <Z> "Z").required(false))
}

#[test]
fn multiple_required_unless_usage_printing() {
    static MULTIPLE_REQUIRED_UNLESS_USAGE: &str =
        "error: The following required arguments were not provided:
    --a <a>
    --b <b>

USAGE:
    test --c <c> --a <a> --b <b>

For more information try --help
";
    let app = App::new("test")
        .arg(
            Arg::new("a")
                .long("a")
                .takes_value(true)
                .required_unless_present("b")
                .conflicts_with("b"),
        )
        .arg(
            Arg::new("b")
                .long("b")
                .takes_value(true)
                .required_unless_present("a")
                .conflicts_with("a"),
        )
        .arg(
            Arg::new("c")
                .long("c")
                .takes_value(true)
                .required_unless_present("d")
                .conflicts_with("d"),
        )
        .arg(
            Arg::new("d")
                .long("d")
                .takes_value(true)
                .required_unless_present("c")
                .conflicts_with("c"),
        );
    assert!(utils::compare_output(
        app,
        "test --c asd",
        MULTIPLE_REQUIRED_UNLESS_USAGE,
        true
    ));
}

#[test]
fn issue_1158_conflicting_requirements() {
    let app = issue_1158_app();

    assert!(utils::compare_output(app, "example id", ISSUE_1158, true));
}

#[test]
fn issue_1158_conflicting_requirements_rev() {
    let res = issue_1158_app().try_get_matches_from(&["", "--config", "some.conf"]);

    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn issue_1643_args_mutually_require_each_other() {
    use clap::*;

    let app = App::new("test")
        .arg(
            Arg::new("relation_id")
                .help("The relation id to get the data from")
                .long("relation-id")
                .short('r')
                .takes_value(true)
                .requires("remote_unit_name"),
        )
        .arg(
            Arg::new("remote_unit_name")
                .help("The name of the remote unit to get data from")
                .long("remote-unit")
                .short('u')
                .takes_value(true)
                .requires("relation_id"),
        );

    app.try_get_matches_from(&["test", "-u", "hello", "-r", "farewell"])
        .unwrap();
}

#[test]
fn short_flag_require_equals_with_minvals_zero() {
    let m = App::new("foo")
        .arg(
            Arg::new("check")
                .short('c')
                .min_values(0)
                .require_equals(true),
        )
        .arg(Arg::new("unique").short('u'))
        .try_get_matches_from(&["foo", "-cu"])
        .unwrap();
    assert!(m.is_present("check"));
    assert!(m.is_present("unique"));
}

#[test]
fn issue_2624() {
    let matches = App::new("foo")
        .arg(
            Arg::new("check")
                .short('c')
                .long("check")
                .require_equals(true)
                .min_values(0)
                .possible_values(["silent", "quiet", "diagnose-first"]),
        )
        .arg(Arg::new("unique").short('u').long("unique"))
        .try_get_matches_from(&["foo", "-cu"])
        .unwrap();
    assert!(matches.is_present("check"));
    assert!(matches.is_present("unique"));
}

#[test]
fn required_unless_all_with_any() {
    let app = App::new("prog")
        .arg(Arg::new("foo").long("foo"))
        .arg(Arg::new("bar").long("bar"))
        .arg(Arg::new("baz").long("baz"))
        .arg(
            Arg::new("flag")
                .long("flag")
                .required_unless_present_any(&["foo"])
                .required_unless_present_all(&["bar", "baz"]),
        );

    let result = app.clone().try_get_matches_from(vec!["myprog"]);

    assert!(result.is_err(), "{:?}", result.unwrap());

    let result = app.clone().try_get_matches_from(vec!["myprog", "--foo"]);

    assert!(result.is_ok(), "{:?}", result.unwrap_err());
    assert!(!result.unwrap().is_present("flag"));

    let result = app
        .clone()
        .try_get_matches_from(vec!["myprog", "--bar", "--baz"]);

    assert!(result.is_ok(), "{:?}", result.unwrap_err());
    assert!(!result.unwrap().is_present("flag"));

    let result = app.try_get_matches_from(vec!["myprog", "--bar"]);

    assert!(result.is_err(), "{:?}", result.unwrap());
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Argument or group 'extra' specified in 'requires*' for 'config' does not exist"]
fn requires_invalid_arg() {
    let _ = App::new("prog")
        .arg(Arg::new("config").requires("extra").long("config"))
        .try_get_matches_from(vec!["", "--config"]);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Argument or group 'extra' specified in 'requires*' for 'config' does not exist"]
fn requires_if_invalid_arg() {
    let _ = App::new("prog")
        .arg(
            Arg::new("config")
                .requires_if("val", "extra")
                .long("config"),
        )
        .try_get_matches_from(vec!["", "--config"]);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Argument or group 'extra' specified in 'required_if_eq*' for 'config' does not exist"]
fn required_if_invalid_arg() {
    let _ = App::new("prog")
        .arg(
            Arg::new("config")
                .required_if_eq("extra", "val")
                .long("config"),
        )
        .try_get_matches_from(vec!["", "--config"]);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Argument or group 'extra' specified in 'required_unless*' for 'config' does not exist"]
fn required_unless_invalid_arg() {
    let _ = App::new("prog")
        .arg(
            Arg::new("config")
                .required_unless_present("extra")
                .long("config"),
        )
        .try_get_matches_from(vec![""]);
}
