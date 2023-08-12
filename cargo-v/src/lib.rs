use std::{error::Error, fs, io};

pub enum VersionLabel {
    Patch,
    Minor,
    Major,
}

pub fn update_version_by_label(
    cargo_toml_content: String,
    version: VersionLabel,
) -> Result<(String, String), Box<dyn Error>> {
    let old_version =
        get_prop_from_cargo_toml(&cargo_toml_content, "[package]", "version").unwrap();
    let (major, minor, patch) = get_version_as_tuple(&old_version);
    update_version(
        cargo_toml_content,
        match version {
            VersionLabel::Patch => {
                format!("{major}.{minor}.{}", increment_version(patch))
            }
            VersionLabel::Minor => {
                format!("{major}.{}.0", increment_version(minor))
            }
            VersionLabel::Major => format!("{}.0.0", increment_version(major)),
        },
    )
}

pub fn update_version(
    cargo_toml_content: String,
    version: String,
) -> Result<(String, String), Box<dyn Error>> {
    let version = version.replace('v', "");
    let old_version =
        get_prop_from_cargo_toml(&cargo_toml_content, "[package]", "version").unwrap();
    verify_new_version_is_grather(&old_version, &version)?;
    Ok((cargo_toml_content.replace(&old_version, &version), version))
}
fn verify_new_version_is_grather(
    old_version: &str,
    new_version: &str,
) -> Result<(), Box<dyn Error>> {
    let (old_major, old_minor, old_patch) = get_version_as_tuple(old_version);
    let (new_major, new_minor, new_patch) = get_version_as_tuple(new_version);
    let old_major: usize = old_major.parse()?;
    let old_minor: usize = old_minor.parse()?;
    let old_patch: usize = old_patch.parse()?;
    let new_major: usize = new_major.parse()?;
    let new_minor: usize = new_minor.parse()?;
    let new_patch: usize = new_patch.parse()?;
    if old_major != new_major {
        if new_major < old_major {
            return Err("You can not set a version lower than the current version")?;
        };
        if new_minor != 0 || new_patch != 0 {
            return Err("You can not set a version lower than the current version")?;
        }
        return Ok(());
    }
    if old_minor != new_minor {
        if new_minor < old_minor {
            return Err("You can not set a version lower than the current version")?;
        }
        if new_patch != 0 {
            return Err("You can not set a version lower than the current version")?;
        }
        return Ok(());
    }
    if new_patch < old_patch {
        return Err("You can not set a version lower than the current version")?;
    }
    if new_patch == old_patch {
        return Err("You can not set a version equal to current version")?;
    }
    Ok(())
}

fn get_prop_from_cargo_toml(
    cargo_toml_content: &str,
    sector: &str,
    prop: &str,
) -> Result<String, Box<dyn Error>> {
    let lines: Vec<&str> = cargo_toml_content.lines().collect();
    let mut should_copy = false;

    for line in lines {
        if line.starts_with('[') {
            should_copy = line.starts_with(sector);
        }
        if should_copy && line.contains(prop) {
            let toml_value = get_value_from_toml_line(line);
            return Ok(toml_value);
        }
    }
    Err(format!(
        "{prop} not found in  sector({sector}) at Cargo.toml"
    ))?
}
fn get_value_from_toml_line(line: &str) -> String {
    line.split('=')
        .last()
        .unwrap()
        .replace('\"', "")
        .trim()
        .to_string()
}
fn get_version_as_tuple(version: &str) -> (&str, &str, &str) {
    let vec: Vec<&str> = version.split('.').collect();
    (vec[0], vec[1], vec[2])
}
fn increment_version(single_version: &str) -> usize {
    single_version.parse::<usize>().unwrap() + 1
}

pub fn read_file(file_name: &str) -> Result<String, Box<dyn Error>> {
    let file = fs::read_to_string(file_name);
    match file {
        Ok(data) => Ok(data),
        Err(err) => Err(err)?,
    }
}

pub fn save_data_in_file(new_file_content: String, file_name: &str) -> io::Result<()> {
    fs::write(file_name, new_file_content)?;
    Ok(())
}

pub fn get_version_from_args_list(
    args: impl Iterator<Item = String>,
) -> Result<String, Box<dyn Error>> {
    let mut args = args.skip(2);
    match args.next() {
        Some(version) => Ok(version),
        None => Err("Version not provided! You must pass the version(patch, minor, major or specific version v1.0.0 by Example)")?,
    }
}
pub enum Operation {
    Patch,
    Major,
    Minor,
    Version(String),
}
impl Operation {
    pub fn from(version: &str) -> Self {
        match version {
            "patch" => Self::Patch,
            "minor" => Self::Minor,
            "major" => Self::Major,
            _ => Self::Version(String::from(version.trim())),
        }
    }
}
#[cfg(test)]
mod test {
    use std::vec::IntoIter;

    use super::*;
    #[test]
    fn ensure_return_version_info_from_args() {
        let input: IntoIter<String> =
            vec![String::from("A"), String::from("B"), String::from("C")].into_iter();
        let expected = String::from("C");
        assert_eq!(expected, get_version_from_args_list(input).unwrap());
    }
    #[test]
    fn ensure_throw_on_version_not_provided() {
        let input: IntoIter<String> = vec![String::from("A"), String::from("B")].into_iter();
        match get_version_from_args_list(input) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err.to_string(),"Version not provided! You must pass the version(patch, minor, major or specific version v1.0.0 by Example)" ),
        }
    }
    #[test]
    fn should_get_version() {
        let input = String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.1\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        let version = get_prop_from_cargo_toml(&input, "[package]", "version").unwrap();
        assert_eq!(version, String::from("0.0.1"));
    }
    #[test]
    fn should_get_version_tuple() {
        let input = String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.1\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        let version_string = get_prop_from_cargo_toml(&input, "[package]", "version").unwrap();
        let version = get_version_as_tuple(&version_string);
        assert_eq!(version, ("0", "0", "1"));
    }
    #[test]
    fn should_update_project_version_by_hand() {
        let input = String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.1\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        let expected = String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.2\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        let version_expected = String::from("0.0.2");
        assert_eq!(
            update_version(input, version_expected.clone()).unwrap(),
            (expected, version_expected)
        );
    }
    #[test]
    fn should_update_project_version_patch() {
        let input = String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.1\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        let expected = String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.2\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        let version_expected = String::from("0.0.2");
        assert_eq!(
            update_version_by_label(input, VersionLabel::Patch).unwrap(),
            (expected, version_expected)
        );
    }
    #[test]
    fn should_update_project_version_minor() {
        let input = String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.1\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        let expected = String::from("[package]\n name = \"cargo-v\"\n version = \"0.1.0\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        let version_expected = String::from("0.1.0");
        assert_eq!(
            update_version_by_label(input, VersionLabel::Minor).unwrap(),
            (expected, version_expected)
        )
    }
    #[test]
    fn should_update_project_version_major() {
        let input = String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.1\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        let expected = String::from("[package]\n name = \"cargo-v\"\n version = \"1.0.0\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        let version_expected = String::from("1.0.0");
        assert_eq!(
            update_version_by_label(input, VersionLabel::Major).unwrap(),
            (expected, version_expected)
        )
    }
    #[test]
    fn should_return_version_from_package_sector_at_toml_file() {
        let input = String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.1\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        let expected = String::from("0.0.1");
        assert_eq!(
            get_prop_from_cargo_toml(&input, "[package]", "version").unwrap(),
            expected
        )
    }
    #[test]
    fn should_return_prop_from_correct_sector_at_toml_file() {
        let input = String::from("[package]\n name = \"any-name\"\n version = \"0.0.1\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        let expected = String::from("any-name");
        assert_eq!(
            get_prop_from_cargo_toml(&input, "[package]", "name").unwrap(),
            expected
        )
    }
    #[test]
    fn should_throw_on_version_patch_passed_lower_than_current() {
        let input = String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.2\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        match update_version(input, "0.0.1".into()) {
            Ok(_) => assert!(false),
            Err(error) => assert_eq!(
                error.to_string(),
                "You can not set a version lower than the current version"
            ),
        };
    }
    #[test]
    fn should_throw_on_version_minor_passed_lower_than_current() {
        let input = String::from("[package]\n name = \"cargo-v\"\n version = \"0.2.2\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        match update_version(input, "0.1.0".into()) {
            Ok(_) => assert!(false),
            Err(error) => assert_eq!(
                error.to_string(),
                "You can not set a version lower than the current version"
            ),
        };
    }
    #[test]
    fn should_throw_on_version_major_passed_lower_than_current() {
        let input = String::from("[package]\n name = \"cargo-v\"\n version = \"2.0.2\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        match update_version(input, "1.0.0".into()) {
            Ok(_) => assert!(false),
            Err(error) => assert_eq!(
                error.to_string(),
                "You can not set a version lower than the current version"
            ),
        };
    }
    #[test]
    fn should_throw_on_old_version_equal_to_current_version() {
        let input = String::from("[package]\n name = \"cargo-v\"\n version = \"2.0.2\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        match update_version(input, "2.0.2".into()) {
            Ok(_) => assert!(false),
            Err(error) => assert_eq!(
                error.to_string(),
                "You can not set a version equal to current version"
            ),
        };
    }
    #[test]
    fn should_throw_on_version_passed_had_negative_number() {
        let input = String::from("[package]\n name = \"cargo-v\"\n version = \"2.0.2\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        match update_version(input, "-2.1.0".into()) {
            Ok(_) => assert!(false),
            Err(error) => assert_eq!(error.to_string(), "invalid digit found in string"),
        };
    }
    #[test]
    fn should_throw_on_version_non_numeric_patch_provided_() {
        let input = String::from("[package]\n name = \"cargo-v\"\n version = \"2.0.2\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        match update_version(input, "2.0.a".into()) {
            Ok(_) => assert!(false),
            Err(error) => assert_eq!(error.to_string(), "invalid digit found in string"),
        };
    }
    #[test]
    fn should_throw_on_version_non_numeric_minor_provided_() {
        let input = String::from("[package]\n name = \"cargo-v\"\n version = \"2.0.2\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        match update_version(input, "2.a.0".into()) {
            Ok(_) => assert!(false),
            Err(error) => assert_eq!(error.to_string(), "invalid digit found in string"),
        };
    }
    #[test]
    fn should_throw_on_version_non_numeric_major_provided_() {
        let input = String::from("[package]\n name = \"cargo-v\"\n version = \"2.0.2\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        match update_version(input, "a.0.0".into()) {
            Ok(_) => assert!(false),
            Err(error) => assert_eq!(error.to_string(), "invalid digit found in string"),
        };
    }
    #[test]
    fn should_return_data_content_from_file() {
        let input = "./exemplo.toml";
        let expected = String::from("[package]\nname = \"cargo-v\"\nversion = \"0.2.24\"\nedition = \"2021\"\n\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n\n[dependencies]\n");

        assert_eq!(expected, read_file(input).unwrap());
    }
    #[test]
    fn should_return_error_on_file_not_found() {
        let input = "./file_inexistent.toml";
        match read_file(input) {
            Ok(_) => assert!(false),
            Err(error) => assert!(error.to_string().contains("No such file or directory")),
        }
    }
    #[test]
    fn should_save_file_content_correctly() {
        let input = String::from("[package]\nname = \"cargo-v\"\nversion = \"0.2.24\"\nedition = \"2021\"\n\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n\n[dependencies]\n");
        let expected = String::from("[package]\nname = \"cargo-v\"\nversion = \"0.2.24\"\nedition = \"2021\"\n\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n\n[dependencies]\n");
        let file_name = "./teste_write.toml";
        let _ = save_data_in_file(input, file_name);
        assert_eq!(expected, read_file(file_name).unwrap());
    }
    #[test]
    fn should_acept_v_prefix() {
        let input = String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.1\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        let expected = String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.2\"\n edition = \"2021\"\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n[dependencies]\n");
        let version_expected = String::from("0.0.2");
        assert_eq!(
            update_version(input, "v0.0.2".into()).unwrap(),
            (expected, version_expected)
        )
    }
    #[test]
    fn should_return_value_from_toml_line() {
        let input = String::from("any_key = \"any_value\"");
        let expected = String::from("any_value");

        assert_eq!(expected, get_value_from_toml_line(&input));
    }
}
