#[macro_export]
macro_rules! error_here {
    () => {
        $crate::error::ErrorWhere::new(line!(), column!(), file!())
    };
}
