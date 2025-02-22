pub mod debug {
    macro_rules! println {
        ($($rest:tt)*) => {
            #[cfg(debug_assertions)]
            std::println!($($rest)*)
        }
    }
    pub(crate) use println;
}