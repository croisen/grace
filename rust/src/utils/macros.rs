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

macro_rules! iter_unwrap {
    ($iter:expr, $errors:expr) => {{
        use $crate::api::ast::unexpected_eof;
        let token = $iter.next();
        if token.is_none() {
            unexpected_eof($iter, $errors);
            return;
        }

        token.unwrap()
    }};

    ($iter:expr, $errors:expr, $return:expr) => {{
        use $crate::api::ast::unexpected_eof;
        let token = $iter.next();
        if token.is_none() {
            unexpected_eof($iter, $errors);
            return $return;
        }

        token.unwrap()
    }};

    (peek($i:expr), $iter:expr, $errors:expr) => {{
        use $crate::api::ast::unexpected_eof;
        let token = $iter.peek($i);
        if token.is_none() {
            unexpected_eof($iter, $errors);
            return;
        }

        token.unwrap()
    }};

    (peek($i:expr), $iter:expr, $errors:expr, $return:expr) => {{
        use $crate::api::ast::unexpected_eof;
        let token = $iter.peek($i);
        if token.is_none() {
            unexpected_eof($iter, $errors);
            return $return;
        }

        token.unwrap()
    }};
}

pub(crate) use all;
pub(crate) use any;
pub(crate) use iter_unwrap;
