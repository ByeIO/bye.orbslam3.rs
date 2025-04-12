use fqdn::{fqdn, Fqdn, FQDN};
use std::{collections::BTreeMap, str::FromStr};

fn get(h: &Fqdn, t: &BTreeMap<FQDN, i32>) -> Option<i32> {
        t.get(h).copied()
}

fn main() {
        let mut t = BTreeMap::new();
        t.insert(fqdn!("3foo.xyz"), 1);
        t.insert(fqdn!("rbar.baz"), 2);

        let fqdn1 = &fqdn::fqdn!("rbar.baz");
        let fqdn2 = fqdn::FQDN::from_str("rbar.baz").unwrap();

        println!("{:?}", t.get(fqdn1));
        println!("{:?}", get(fqdn1, &t));
        println!("{:?}", get(&fqdn2, &t));
        println!("{:?}", get(&fqdn!("rbar.baz"), &t));
}