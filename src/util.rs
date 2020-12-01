use std::path::PathBuf;

pub(crate) fn load_input(path: &str) -> String {
    let file_name = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join(path);

    std::fs::read_to_string(file_name).expect("Something went wrong reading the file")
}

pub(crate) fn lines_to_vec<F>(input: &str) -> Vec<F>
where
    F: core::str::FromStr,
    F::Err: std::fmt::Debug,
{
    input
        .lines()
        .map(|v| v.parse::<F>().unwrap())
        .collect::<Vec<_>>()
}
