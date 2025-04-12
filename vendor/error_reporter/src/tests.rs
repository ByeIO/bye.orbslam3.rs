use crate::Report;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
struct E {
    a: &'static str,
    b: Option<Box<dyn Error>>,
}

impl Display for E {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.a)
    }
}

impl Error for E {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.b.as_ref().map(|e| &**e)
    }
}

#[test]
fn test1() {
    const SINGLE: &str = "a";
    const MULTI: &str = "a";
    let a = E { a: "a", b: None };
    let report = Report::from(a);
    assert_eq!(report.to_string(), SINGLE,);
    assert_eq!(report.pretty(true).to_string(), MULTI,);
}

#[test]
fn test2() {
    const SINGLE: &str = "b: a";
    const MULTI: &str = "\
b

Caused by:
      a";
    let a = E { a: "a", b: None };
    let b = E {
        a: "b",
        b: Some(Box::new(a)),
    };
    let report = Report::from(b);
    assert_eq!(report.to_string(), SINGLE,);
    assert_eq!(report.pretty(true).to_string(), MULTI,);
}

#[test]
fn test3() {
    const SINGLE: &str = "c: b: a";
    const MULTI: &str = "\
c

Caused by:
   0: b
   1: a";
    let a = E { a: "a", b: None };
    let b = E {
        a: "b",
        b: Some(Box::new(a)),
    };
    let c = E {
        a: "c",
        b: Some(Box::new(b)),
    };
    let report = Report::from(c);
    assert_eq!(report.to_string(), SINGLE,);
    assert_eq!(report.pretty(true).to_string(), MULTI,);
}
