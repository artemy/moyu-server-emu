use uuid::Uuid;

pub trait ToStringWithoutDashes {
    fn to_string_without_dashes(&self) -> String;
}

impl ToStringWithoutDashes for Uuid {
    fn to_string_without_dashes(&self) -> String {
        self.to_string().replace('-', "")
    }
}

pub const DATE_FORMAT: &str = "%Y-%m-%d-%H%M%S";
