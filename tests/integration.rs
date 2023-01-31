use assert_cmd::Command;

#[test]
fn case1() {
    let mut bin = bin();

    let mut path = fixtures();
    path.push("case1");
    bin.args(["--dir", path.display().to_string().as_str()]);

    insta::assert_snapshot!(&String::from_utf8_lossy(
        bin.assert().success().get_output().stdout.as_slice(),
    ));
}

// asser_cmd create temporary directory for testing command
// so we need absolute path.
fn fixtures() -> std::path::PathBuf {
    std::fs::canonicalize("./tests/fixtures").unwrap()
}
fn bin() -> Command {
    Command::cargo_bin("structiagram").unwrap()
}
