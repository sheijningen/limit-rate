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

    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn runs_once() {
        let mut counter = Counter::new();
        limit_rate!(counter.increment(), Duration::from_secs(1));
        assert_eq!(counter.get_count(), 1);
    }

    #[test]
    fn calls_after_duration() {
        let mut counter = Counter::new();
        for _ in 0..5 {
            limit_rate!(counter.increment(), Duration::from_millis(20));
            sleep(Duration::from_millis(40))
        }
        assert_eq!(counter.get_count(), 5);
    }

    #[test]
    fn doesnt_call_before_duration() {
        let mut counter = Counter::new();
        for _ in 0..300 {
            limit_rate!(counter.increment(), Duration::from_secs(30));
        }
        assert_eq!(counter.get_count(), 1);
    }

    #[derive(Debug, Clone)]
    pub struct Counter {
        count: i32,
    }

    impl Counter {
        pub fn new() -> Self {
            Counter { count: 0 }
        }

        pub fn increment(&mut self) {
            self.count += 1;
        }

        pub fn get_count(&self) -> i32 {
            self.count
        }
    }
}
