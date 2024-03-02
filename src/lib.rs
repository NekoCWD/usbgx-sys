#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/* IDK how i can test lib. For configfs writes you need to be root. Also i can test it only on my phone lol */