use crate::Config;

pub(crate) struct Context<'a> {
    pub tailwind_config: &'a Config,
}
