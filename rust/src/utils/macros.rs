#[macro_export]
macro_rules! all {
    ($($arg:expr),*) => {{
        let mut truthy = false;
        $(
            if ! $arg {
                truthy = false;
            }
        )*

        truthy
    }};
}

#[macro_export]
macro_rules! any {
    ($($arg:expr),*) => {{
        let mut truthy = false;
        $(
            if $arg {
                truthy = true;
            }
        )*

        truthy
        }};
}

pub(crate) use all;
pub(crate) use any;
