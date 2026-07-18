#[boltffi::export]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[boltffi::export]
pub fn init_logger() {
    #[cfg(target_os = "android")]
    {
        use android_logger::Config;
        use log::{LevelFilter, info};

        android_logger::init_once(
            Config::default()
                .with_max_level(LevelFilter::Trace)
                .with_tag("PhoneBridgeLib"),
        );
        info!("Rust logging initialized for Android");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
