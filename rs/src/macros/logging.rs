#[macro_export]
macro_rules! log_elapsed {
    (@measure $operation:expr, $message:expr) => {{
        if cfg!(debug_assertions) {
            let start = std::time::Instant::now();
            let result = $operation();
            let elapsed = start.elapsed();
            if elapsed.as_secs() > 2 {
                log::debug!("Function {}: Elapsed {:?}", $message, elapsed);
            }
            result
        } else {
            $operation()
        }
    }};
    ($operation:expr, $message:expr) => {{
        $crate::log_elapsed!(@measure || $operation(), $message)
    }};
    ($operation:expr) => {{
        $crate::log_elapsed!(@measure || $operation, stringify!($operation))
    }};
}
