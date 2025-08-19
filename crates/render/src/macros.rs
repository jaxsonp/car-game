#[macro_export]
macro_rules! load_obj {
    ($file:literal) => {
        include!(concat!(env!("OUT_DIR"), "/", $file, ".rs"))
    };
}
