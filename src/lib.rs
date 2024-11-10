#[macro_export]
macro_rules! limit_rate {
    ($expr:expr, $duration:expr) => {{
        thread_local! {
            static LAST_EXECUTION_TIME: std::cell::Cell<Option<std::time::Instant>> = std::cell::Cell::new(None);
        }

        let _duration: std::time::Duration = $duration;  // Gives a more intuitive error in case of a incorrect duration arg
        if LAST_EXECUTION_TIME.with(|last_exec| {
            let now = std::time::Instant::now();
            if last_exec.get().map_or(true, |last| now.duration_since(last) >= _duration) {
                last_exec.set(Some(now));
                true
            } else {
                false
            }
        }) {
            $expr
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        for _ in 0..3 {
            limit_rate!(
                eprintln!("AAAAAAAAAAAA"),
                std::time::Duration::from_millis(0)
            )
        }
    }
}
