//! Every function documents its timing: the helpers of these crates sit on the
//! password's path, so all of them are checked, private ones included. Trait
//! declarations, which end in `;`, make no promise of their own.

/// Whether `line` declares a function, whatever its visibility.
fn declares_function(line: &str) -> bool {
    let line = line
        .strip_prefix("pub(crate) ")
        .or_else(|| line.strip_prefix("pub(super) "))
        .or_else(|| line.strip_prefix("pub "))
        .unwrap_or(line);
    let line = line.strip_prefix("const ").unwrap_or(line);
    line.starts_with("fn ")
}

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
        if declares_function(line) && !line.ends_with(';') {
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
fn the_scanner_requires_exactly_one_timing_classification_on_each_function() {
    let fixture = "/// Constant time.
pub fn documented() {}
pub fn undocumented() {}
/// VARIABLE TIME.
#[inline]
pub(super) const fn id() {}
/// Constant time and variable time.
fn helper() {}
/// Wrapped, constant
/// time.
pub(crate) fn wrapped() {}
fn declared(&self) -> u32;
let value = fnord;
#[cfg(test)]
mod tests {
    fn helper() {}
}
";
    let (checked, missing) = missing_timing_docs(fixture);
    assert_eq!(checked, 5);
    assert_eq!(missing, ["pub fn undocumented() {}", "fn helper() {}"]);
}

#[test]
fn every_function_has_an_unambiguous_timing_doc() {
    for (name, source, minimum) in [
        (
            "argon2/argon2_error.rs",
            include_str!("../src/argon2/argon2_error.rs"),
            1,
        ),
        (
            "argon2/argon2_params_ref.rs",
            include_str!("../src/argon2/argon2_params_ref.rs"),
            15,
        ),
        (
            "argon2/argon2_type.rs",
            include_str!("../src/argon2/argon2_type.rs"),
            1,
        ),
        (
            "argon2/argon2_version.rs",
            include_str!("../src/argon2/argon2_version.rs"),
            1,
        ),
        (
            "argon2/block.rs",
            include_str!("../src/argon2/block.rs"),
            13,
        ),
        (
            "argon2/generator.rs",
            include_str!("../src/argon2/generator.rs"),
            9,
        ),
        ("argon2/hash.rs", include_str!("../src/argon2/hash.rs"), 1),
        (
            "bcrypt/base64.rs",
            include_str!("../src/bcrypt/base64.rs"),
            2,
        ),
        (
            "bcrypt/bcrypt_error.rs",
            include_str!("../src/bcrypt/bcrypt_error.rs"),
            1,
        ),
        (
            "bcrypt/bcrypt_version.rs",
            include_str!("../src/bcrypt/bcrypt_version.rs"),
            2,
        ),
        (
            "bcrypt/eks_blowfish.rs",
            include_str!("../src/bcrypt/eks_blowfish.rs"),
            8,
        ),
        (
            "bcrypt/generator.rs",
            include_str!("../src/bcrypt/generator.rs"),
            2,
        ),
        (
            "bcrypt/open_bsd_bcrypt.rs",
            include_str!("../src/bcrypt/open_bsd_bcrypt.rs"),
            3,
        ),
        (
            "scrypt/generator.rs",
            include_str!("../src/scrypt/generator.rs"),
            2,
        ),
        (
            "scrypt/ro_mix.rs",
            include_str!("../src/scrypt/ro_mix.rs"),
            2,
        ),
        ("scrypt/salsa.rs", include_str!("../src/scrypt/salsa.rs"), 2),
        (
            "scrypt/scrypt_error.rs",
            include_str!("../src/scrypt/scrypt_error.rs"),
            2,
        ),
    ] {
        let (checked, missing) = missing_timing_docs(source);
        assert!(
            checked >= minimum,
            "only {checked} functions scanned in {name}"
        );
        assert!(
            missing.is_empty(),
            "missing timing docs in {name}: {missing:?}"
        );
    }
}
