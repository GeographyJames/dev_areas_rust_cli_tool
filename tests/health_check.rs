use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn fails_if_no_arguments_given() {
    let mut cmd = Command::cargo_bin("dev_areas").unwrap();
    cmd.assert().failure();
}

#[test]
fn fails_if_argument_is_not_valid_file() {
    let mut cmd = Command::cargo_bin("dev_areas").unwrap();
    cmd.arg("a_file");
    cmd.assert().failure();
}
