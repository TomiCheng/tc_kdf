//! Every public function and every method a type implements documents its
//! timing. Trait declarations, which end in `;`, make no promise of their own.

/// Declaration prefixes whose doc comments must state their timing.
const DECLARATIONS: &[&str] = &[
    "pub fn ",
    "pub const fn ",
    "pub(crate) fn ",
    "fn init(",
    "fn generate_bytes(",
    "fn fmt(",
    "fn source(",
    "fn drop(",
    "fn shared_secret(",
    "fn iv(",
    "fn ikm(",
    "fn salt(",
    "fn info(",
    "fn skip_extract(",
    "fn ki(",
    "fn fixed_input",
    "fn counter_bits(",
];

fn missing_timing_docs(source: &str) -> (usize, Vec<String>) {
    let mut docs = String::new();
    let mut attribute_depth = 0_i32;
    let mut checked = 0;
    let mut missing = Vec::new();
    for line in source.lines().map(str::trim) {
        // Helpers inside test modules are not part of the contract.
        if line == "#[cfg(test)]" {
            break;
        }
        if let Some(comment) = line.strip_prefix("///") {
            // Lines join with a space, so that a classification may wrap.
            docs.push_str(comment.trim());
            docs.push(' ');
            continue;
        }
        if line.starts_with("#[") || attribute_depth > 0 {
            attribute_depth += line.chars().filter(|&c| c == '[').count() as i32;
            attribute_depth -= line.chars().filter(|&c| c == ']').count() as i32;
            continue;
        }
        if line.is_empty() || line.starts_with("//") {
            continue;
        }
        let declaration = DECLARATIONS.iter().any(|prefix| line.starts_with(prefix));
        if declaration && !line.ends_with(';') {
            checked += 1;
            let docs = docs.to_ascii_lowercase();
            if docs.contains("constant time") == docs.contains("variable time") {
                missing.push(line.to_owned());
            }
        }
        docs.clear();
    }
    (checked, missing)
}

#[test]
fn the_scanner_requires_exactly_one_timing_classification_on_each_declaration() {
    let fixture = "\
/// Constant time.
pub fn documented() {}
pub fn undocumented() {}
/// VARIABLE TIME.
#[inline]
fn init() {}
/// Constant time and variable time.
fn drop() {}
/// Wrapped, constant
/// time.
fn fmt() {}
fn ki(&self) -> &[u8];
fn helper() {}
#[cfg(test)]
mod tests {
    fn init() {}
}
";
    let (checked, missing) = missing_timing_docs(fixture);
    assert_eq!(checked, 5);
    assert_eq!(missing, ["pub fn undocumented() {}", "fn drop() {}"]);
}

#[test]
fn every_public_api_has_an_unambiguous_timing_doc() {
    for (name, source, minimum) in [
        (
            "errors/kdf_error.rs",
            include_str!("../src/errors/kdf_error.rs"),
            2,
        ),
        (
            "hkdf/generator.rs",
            include_str!("../src/hkdf/generator.rs"),
            4,
        ),
        (
            "hkdf/params_owned.rs",
            include_str!("../src/hkdf/params_owned.rs"),
            7,
        ),
        (
            "hkdf/params_ref.rs",
            include_str!("../src/hkdf/params_ref.rs"),
            6,
        ),
        ("kdf/base_kdf.rs", include_str!("../src/kdf/base_kdf.rs"), 4),
        (
            "kdf/concatenation_kdf.rs",
            include_str!("../src/kdf/concatenation_kdf.rs"),
            4,
        ),
        (
            "kdf/params_owned.rs",
            include_str!("../src/kdf/params_owned.rs"),
            5,
        ),
        (
            "kdf/params_ref.rs",
            include_str!("../src/kdf/params_ref.rs"),
            4,
        ),
        (
            "sp800_108/counter.rs",
            include_str!("../src/sp800_108/counter.rs"),
            4,
        ),
        (
            "sp800_108/double_pipeline.rs",
            include_str!("../src/sp800_108/double_pipeline.rs"),
            4,
        ),
        (
            "sp800_108/feedback.rs",
            include_str!("../src/sp800_108/feedback.rs"),
            4,
        ),
        (
            "sp800_108/init_error.rs",
            include_str!("../src/sp800_108/init_error.rs"),
            2,
        ),
        (
            "sp800_108/params_owned.rs",
            include_str!("../src/sp800_108/params_owned.rs"),
            20,
        ),
        (
            "sp800_108/params_ref.rs",
            include_str!("../src/sp800_108/params_ref.rs"),
            17,
        ),
    ] {
        let (checked, missing) = missing_timing_docs(source);
        assert!(
            checked >= minimum,
            "only {checked} declarations scanned in {name}"
        );
        assert!(
            missing.is_empty(),
            "missing timing docs in {name}: {missing:?}"
        );
    }
}
