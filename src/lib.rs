#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::cmp::Ordering;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


impl PartialEq<Janet> for Janet {
    fn eq(&self, other: &Janet) -> bool {
        unsafe { janet_equals(*self, *other) != 0 }
    }
}

impl Eq for Janet {}

impl PartialOrd<Janet> for Janet {
    fn partial_cmp(&self, other: &Janet) -> Option<Ordering> {
        let res = unsafe { janet_compare(*self,*other) };

        Some(match res {
            -1 => Ordering::Less,
            0 => Ordering::Equal,
            1 => Ordering::Greater,
            _ => return None
        })
    }
}

impl Ord for Janet {
    fn cmp(&self, other: &Self) -> Ordering {
        let res = unsafe { janet_compare(*self,*other) };

        match res {
            -1 => Ordering::Less,
            0 => Ordering::Equal,
            1 => Ordering::Greater,
            _ => unreachable!()
        }
    }
}
