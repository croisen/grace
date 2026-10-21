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

macro_rules! skip_match {
    ($iter:expr, $pat:pat $(,)?) => {
        $iter.skip_while(|t| matches!(t.kind, $pat))
    };
}

macro_rules! skip_reverse {
    ($iter:expr, $pat:pat $(,)?) => {
        $iter.skip_while(|t| !matches!(t.kind, $pat))
    };
}

pub(crate) use all;
pub(crate) use any;
pub(crate) use skip_match;
pub(crate) use skip_reverse;
