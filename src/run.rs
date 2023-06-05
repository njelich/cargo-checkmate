use std::ffi::OsString;

/// Run the cli main process
pub fn run() -> anyhow::Result<()> {
    run_with_args(std::env::args())
}

pub fn run_with_args<I, T>(it: I) -> anyhow::Result<()>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    use crate::executable::Executable;
    use crate::options::Options;

    crate::cdcrate::change_directory_to_crate_root()?;
    let opts = Options::parse_args(it);
    opts.execute()
}

#[cfg(test)]
mod tests;
