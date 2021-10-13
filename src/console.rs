pub mod interface {
    pub use core::fmt;

    // why create our own? 
    pub trait Write {
        /// Write a Rust format string.
        fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;
    }

    pub trait Statistics {
        /// Return the number of characters written.
        fn chars_written(&self) -> usize {
            0
        }
    }

    // Trait alias for full-fledged console.
    pub trait All = Write + Statistics;
    
}
