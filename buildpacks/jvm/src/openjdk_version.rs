use crate::util::zip_longest;
use nom::Finish;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub(crate) enum OpenJdkVersion {
    Legacy {
        major: u32,
        update: u32,
    },
    Jep322 {
        major: u32,
        remaining_elements: Vec<u32>,
        suffix: Jep322VnumSuffix,
    },
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub(crate) struct Jep322VnumSuffix {
    pre: Option<String>,
    build: Option<u32>,
    opt: Option<String>,
}

impl OpenJdkVersion {
    pub(crate) fn major(&self) -> u32 {
        match self {
            OpenJdkVersion::Legacy { major, .. } | OpenJdkVersion::Jep322 { major, .. } => *major,
        }
    }
}

impl PartialEq<Self> for OpenJdkVersion {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd<Self> for OpenJdkVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.major().cmp(&other.major()) {
            Ordering::Equal => match (self, other) {
                // Compare legacy versions beyond the major version
                (
                    OpenJdkVersion::Legacy { update: a, .. },
                    OpenJdkVersion::Legacy { update: b, .. },
                ) => Some(a.cmp(b)),
                // Compare JEP 322 versions beyond the major version
                (
                    OpenJdkVersion::Jep322 {
                        remaining_elements: a,
                        ..
                    },
                    OpenJdkVersion::Jep322 {
                        remaining_elements: b,
                        ..
                    },
                ) => Some(
                    zip_longest(a.iter(), b.iter())
                        .map(|item| match item {
                            (Some(element_a), Some(element_b)) => element_a.cmp(element_b),
                            (Some(_), None) => Ordering::Greater,
                            (None, Some(_)) => Ordering::Less,
                            // While this should never happen (due to zip_longest's implementation),
                            // we consider two missing elements to be equal instead of panicking.
                            (None, None) => Ordering::Equal,
                        })
                        .find(|ordering| *ordering != Ordering::Equal)
                        .unwrap_or(Ordering::Equal),
                ),
                // Legacy and JEP 322 versions cannot be compared beyond major versions
                _ => None,
            },
            major_version_ordering => Some(major_version_ordering),
        }
    }
}

impl Display for OpenJdkVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenJdkVersion::Legacy { major, update } => {
                // We use this format as it is the format Heroku historically has used. There might
                // be code that relies on this format and cannot work with the `$MAJORu$UPDATE`
                // format.
                f.write_fmt(format_args!("1.{major}.0_{update}"))
            }
            OpenJdkVersion::Jep322 {
                major,
                remaining_elements,
                ..
            } => f.write_fmt(format_args!(
                "{major}.{}",
                remaining_elements
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(".")
            )),
        }
    }
}

impl Serialize for OpenJdkVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl FromStr for OpenJdkVersion {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse::vstr(s).finish() {
            Ok((_, version)) => Ok(version),
            Err(nom::error::Error { input, code }) => Err(nom::error::Error {
                input: String::from(input),
                code,
            }),
        }
    }
}

impl<'de> Deserialize<'de> for OpenJdkVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)
            .and_then(|string| string.parse::<Self>().map_err(serde::de::Error::custom))
    }
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum OpenJdkParseError {
    #[error("Unexpected version elements in legacy version string: {0:?}")]
    UnexpectedLegacyVnum(Vec<u32>),
}

mod parse {
    use super::{Jep322VnumSuffix, OpenJdkParseError, OpenJdkVersion};
    use nom::branch::alt;
    use nom::bytes::complete::{tag, take_while1};
    use nom::character::complete::{alphanumeric1, char, digit1};
    use nom::combinator::{eof, map, map_res, opt, success};
    use nom::multi::separated_list1;
    use nom::sequence::{preceded, terminated, tuple};
    use nom::IResult;

    pub(super) fn vstr(input: &str) -> IResult<&str, OpenJdkVersion> {
        terminated(alt((legacy_vstr, jep_322_vstr)), eof)(input)
    }

    fn jep_322_vstr(input: &str) -> IResult<&str, OpenJdkVersion> {
        let major_remaining_elements_vnum = map(vnum, |vnum| match vnum.as_slice() {
            [] => unreachable!("vnum parser should fail on zero elements"),
            [major, rest @ ..] => (*major, Vec::from(rest)),
        });

        map(
            tuple((major_remaining_elements_vnum, jep_322_vnum_suffix)),
            |((major, remaining_elements), suffix)| OpenJdkVersion::Jep322 {
                major,
                remaining_elements,
                suffix,
            },
        )(input)
    }

    fn legacy_vstr(input: &str) -> IResult<&str, OpenJdkVersion> {
        // The `vnum` portion of a legacy OpenJDK versions string works differently from JEP 322
        // `vnum`. For legacy versions, only the major version is encoded (in two different forms).
        let major_version_vnum = map_res(vnum, |vnum| match vnum.as_slice() {
            [] => unreachable!("vnum parser should fail on zero elements"),
            [major] | [1, major /* No trailing zero, removed by vnum */] => Ok(*major),
            unexpected => Err(OpenJdkParseError::UnexpectedLegacyVnum(Vec::from(
                unexpected,
            ))),
        });

        map(
            tuple((major_version_vnum, legacy_vnum_suffix)),
            |(major, update_suffix)| OpenJdkVersion::Legacy {
                major,
                update: update_suffix,
            },
        )(input)
    }

    fn vnum(input: &str) -> IResult<&str, Vec<u32>> {
        map(separated_list1(char('.'), u32_string), |elements| {
            // Normalize by removing trailing zeroes as defined in JEP 322.
            let mut elements = elements
                .into_iter()
                .rev()
                .skip_while(|element| *element == 0)
                .collect::<Vec<u32>>();

            elements.reverse();
            elements
        })(input)
    }

    fn legacy_vnum_suffix(input: &str) -> IResult<&str, u32> {
        preceded(alt((char('u'), char('_'))), u32_string)(input)
    }

    fn jep_322_vnum_suffix(input: &str) -> IResult<&str, Jep322VnumSuffix> {
        let any_jep_322_vnum_suffix_format = alt((
            tuple((
                opt(preceded(char('-'), alphanumeric1)),
                map(preceded(char('+'), u32_string), Some),
                opt(preceded(char('-'), jep_322_vnum_suffix_opt)),
            )),
            tuple((
                map(preceded(char('-'), alphanumeric1), Some),
                success(None),
                opt(preceded(char('-'), jep_322_vnum_suffix_opt)),
            )),
            tuple((
                success(None),
                success(None),
                opt(preceded(tag("+-"), jep_322_vnum_suffix_opt)),
            )),
        ));

        map(any_jep_322_vnum_suffix_format, |(pre, build, opt)| {
            Jep322VnumSuffix {
                pre: pre.map(String::from),
                build,
                opt: opt.map(String::from),
            }
        })(input)
    }

    fn jep_322_vnum_suffix_opt(input: &str) -> IResult<&str, &str> {
        take_while1(|c: char| c.is_ascii_alphanumeric() || c == '.' || c == '-')(input)
    }

    fn u32_string(input: &str) -> IResult<&str, u32> {
        map_res(digit1, str::parse::<u32>)(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::openjdk_version::{Jep322VnumSuffix, OpenJdkVersion};

    #[test]
    fn test_legacy() {
        assert_eq!(
            "8u100".parse::<OpenJdkVersion>().unwrap(),
            OpenJdkVersion::Legacy {
                major: 8,
                update: 100
            }
        );

        assert_eq!(
            "1.8.0_412".parse::<OpenJdkVersion>().unwrap(),
            OpenJdkVersion::Legacy {
                major: 8,
                update: 412
            }
        );
    }

    #[test]
    fn test_jep_322() {
        assert_eq!(
            "18.0.2.1".parse::<OpenJdkVersion>().unwrap(),
            OpenJdkVersion::Jep322 {
                major: 18,
                remaining_elements: vec![0, 2, 1],
                suffix: Jep322VnumSuffix {
                    pre: None,
                    build: None,
                    opt: None,
                }
            }
        );

        assert_eq!(
            "18+12-ga".parse::<OpenJdkVersion>().unwrap(),
            OpenJdkVersion::Jep322 {
                major: 18,
                remaining_elements: vec![],
                suffix: Jep322VnumSuffix {
                    pre: None,
                    build: Some(12),
                    opt: Some(String::from("ga")),
                }
            }
        );
    }

    #[test]
    fn test_ord_less() {
        let values = [
            ("10.0.4", "10.1.2"),
            ("10.0.2", "10.0.2.1"),
            ("11", "12"),
            ("8u123", "11.0.1"),
            ("8u281", "8u361"),
            ("11", "11.0.1"),
        ];

        for (a, b) in values {
            let a = a.parse::<OpenJdkVersion>().unwrap();
            let b = b.parse::<OpenJdkVersion>().unwrap();
            assert!(a < b);
        }
    }

    #[test]
    fn test_ord_equal() {
        let values = [
            ("8u100", "1.8.0_100"),
            ("11", "11.0.0"),
            ("22.0.1.0", "22.0.1.0"),
            ("22-ga", "22.0.0-ga"),
        ];

        for (a, b) in values {
            let a = a.parse::<OpenJdkVersion>().unwrap();
            let b = b.parse::<OpenJdkVersion>().unwrap();
            assert_eq!(a, b);
        }
    }
}
