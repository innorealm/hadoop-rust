use crate::common::conf::Configuration;

pub trait Configurable {
    fn set_conf(conf: &Configuration);

    fn get_conf<'a>() -> &'a Configuration;
}
