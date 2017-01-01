use std::rc::Rc;
use std::ffi::{OsStr, OsString};

use vec_map::VecMap;

use Arg;

#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub struct Valued<'a, 'b>
    where 'a: 'b
{
    pub possible_vals: Option<Vec<&'b str>>,
    pub val_names: Option<VecMap<&'b str>>,
    pub num_vals: Option<u64>,
    pub max_vals: Option<u64>,
    pub min_vals: Option<u64>,
    pub validator: Option<Rc<Fn(String) -> Result<(), String>>>,
    pub validator_os: Option<Rc<Fn(&OsStr) -> Result<(), OsString>>>,
    pub val_delim: Option<char>,
    pub default_val: Option<&'a str>,
    pub default_vals_ifs: Option<VecMap<(&'a str, Option<&'b str>, &'b str)>>,
    pub terminator: Option<&'b str>,
}

impl<'n, 'e> Default for Valued<'n, 'e> {
    fn default() -> Self {
        Valued {
            possible_vals: None,
            num_vals: None,
            min_vals: None,
            max_vals: None,
            val_names: None,
            validator: None,
            validator_os: None,
            val_delim: Some(','),
            default_val: None,
            default_vals_ifs: None,
            terminator: None,
        }
    }
}

impl<'n, 'e, 'z> From<&'z Arg<'n, 'e>> for Valued<'n, 'e> {
    fn from(a: &'z Arg<'n, 'e>) -> Self {
        let mut v = Valued {
            possible_vals: a.possible_vals.clone(),
            num_vals: a.num_vals,
            min_vals: a.min_vals,
            max_vals: a.max_vals,
            val_names: a.val_names.clone(),
            validator: a.validator.clone(),
            validator_os: a.validator_os.clone(),
            val_delim: a.val_delim,
            default_val: a.default_val,
            default_vals_ifs: a.default_vals_ifs.clone(),
            terminator: a.val_terminator.clone(),
        };
        if let Some(ref vec) = a.val_names {
            if vec.len() > 1 {
                v.num_vals = Some(vec.len() as u64);
            }
        }
        if let Some(ref vec) = a.val_names {
            if vec.len() > 1 {
                v.num_vals = Some(vec.len() as u64);
            }
        }
        v
    }
}
