pub trait Enum<T> {
    fn get_string(input_enum: T) -> String;
    fn get_enum(input_string: &str) -> T;
}
