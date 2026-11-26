//! Every public function and every method a type implements documents its
//! timing. Trait declarations, which end in `;`, make no promise of their own.

/// Declaration prefixes whose doc comments must state their timing.
const DECLARATIONS: &[&str] = &[
    "pub fn ",
    "pub const fn ",
    "pub(crate) fn ",
    "fn init(",
    "fn derive_key(",
    "fn derive_key_and_iv(",
    "fn derive_mac_key(",
    "fn derive(",
    "fn repeat_fill(",
    "fn add_one_more(",
    "fn fmt(",
    "fn source(",
    "fn drop(",
    "fn password(",
    "fn salt(",
    "fn iteration_count(",
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
    let fixture = "/// Constant time.
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
fn salt(&self) -> &[u8];
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
            "errors/pbe_error.rs",
            include_str!("../src/errors/pbe_error.rs"),
            2,
        ),
        ("openssl_pbe.rs", include_str!("../src/openssl_pbe.rs"), 6),
        (
            "params/pbe_params_owned.rs",
            include_str!("../src/params/pbe_params_owned.rs"),
            5,
        ),
        (
            "params/pbe_params_ref.rs",
            include_str!("../src/params/pbe_params_ref.rs"),
            4,
        ),
        ("password.rs", include_str!("../src/password.rs"), 3),
        ("pkcs12.rs", include_str!("../src/pkcs12.rs"), 8),
        ("pkcs5_s1.rs", include_str!("../src/pkcs5_s1.rs"), 6),
        ("pkcs5_s2.rs", include_str!("../src/pkcs5_s2.rs"), 6),
        ("state.rs", include_str!("../src/state.rs"), 1),
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
