#![allow(dead_code)]
extern crate clap;
extern crate yaml_rust;

use clap::App;
use clap::ArgMatches;

use yaml_rust::Yaml;

#[derive(Debug)]
pub struct Argparse {
    yaml: Yaml,
}

impl Default for Argparse {
    fn default() -> Argparse {
        Argparse {
            yaml: Yaml::from_str(""),
        }
    }
}

impl<'a> Argparse {
    pub fn init(yaml: Yaml) -> Argparse {
        let args = Argparse {
            yaml: yaml.to_owned(),
        };
        return args;
    }

    pub fn matches(&self) -> ArgMatches {
        return App::from_yaml(&self.yaml).get_matches();
    }

    pub fn bool(&self, argname: &str) -> bool {
        let m = self.matches();
        let mut r = false;
        let n = &m.occurrences_of(argname);
        if n > &0 {
            r = true;
        }
        return r;
    }

    pub fn occurence(&self, argname: &str) -> u64 {
        let m = self.matches();
        let i = m.occurrences_of(argname);
        let j = i.to_owned();
        return j;
    }

    // return types of values
    pub fn val_str(&self, argname: &str) -> String {
        let m = self.matches();
        let v = match m.value_of(argname) {
            Some(x) => x,
            None => "",
        };
        let val = v.to_string();
        return val;
    }

    pub fn val_op_str(&self, argname: &str) -> Option<String> {
        if self.bool(argname) == true {
            Some(self.val_str(argname))
        } else {
            None
        }
    }

    pub fn val_uint(&self, argname: &str) -> u64 {
        let m = self.matches();
        let v = match m.value_of(argname) {
            Some(x) => x,
            None => "0",
        };
        let val = v.parse::<u64>().unwrap();
        return val;
    }

    pub fn val_op_uint(&self, argname: &str) -> Option<u64> {
        if self.bool(argname) == true {
            Some(self.val_uint(argname))
        } else {
            None
        }
    }

    pub fn val_usize(&self, argname: &str) -> usize {
        let m = self.matches();
        let v = m.value_of(argname).unwrap();
        let val = v.parse::<usize>().unwrap();
        return val;
    }

    pub fn val_op_usize(&self, argname: &str) -> Option<usize> {
        if self.bool(argname) == true {
            Some(self.val_usize(argname))
        } else {
            None
        }
    }
}
