#[macro_export]
macro_rules! log_assert {
    (
        $assertion:expr,
        $($message:tt)+
    ) => ({
        match ($assertion, ::core::format_args!($($message)+)) {
            (assertion, message) => {
                if !assertion {
                    ::log::error!("{}", message);
                    ::core::panic!("{}", message);
                }
            }
        }
    });

    (
        $assertion:expr $(,)?
    ) => (
        $crate::log_assert! {
            $assertion, "Assertion failed: `{}`", ::core::stringify!($assertion),
        }
    );
}
