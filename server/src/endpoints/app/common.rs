use crate::endpoints::errors::AppError;
use crate::endpoints::errors::AppError::AuthError;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Deserialize)]
pub struct AppAuth {
    pub app_id: String,
    pub auth: String,
    #[serde(rename = "serviceToken")]
    pub service_token: String,
}

impl Display for AppAuth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "app_id: [{}], auth: [{}], service_token: [{}]",
            self.app_id, self.auth, self.service_token
        )
    }
}

pub trait Auth {
    fn auth(&self) -> &str;
    fn service_token(&self) -> &str;
    fn auth_value(&self) -> String;

    fn md5(&self) -> String {
        let auth_value = self.auth_value();
        let md5_source = format!("{}{}", auth_value, std::env::var("MD5_KEY_1").unwrap());
        format!("{:x}", md5::compute(md5_source))
    }
    fn md5_old(&self) -> String {
        let auth_value = self.auth_value();
        let md5_source = format!("{}{}", auth_value, std::env::var("MD5_KEY_2").unwrap());
        format!("{:x}", md5::compute(md5_source))
    }

    fn validate_auth(&self) -> Result<(), AppError> {
        let md5 = self.md5();
        if md5 == self.auth() {
            Ok(())
        } else {
            log::warn!(
                "{}",
                format!("Invalid auth: [{}], computed: [{}]", self.auth(), md5)
            );
            Err(AuthError)
        }
    }
    fn validate_auth_old(&self) -> Result<(), AppError> {
        let md5 = self.md5_old();
        if md5 == self.auth() {
            Ok(())
        } else {
            log::warn!(
                "{}",
                format!("Invalid auth: [{}], computed: [{}]", self.auth(), md5)
            );
            Err(AuthError)
        }
    }
}

impl Auth for AppAuth {
    fn auth(&self) -> &str {
        self.auth.as_str()
    }
    fn service_token(&self) -> &str {
        self.service_token.as_str()
    }

    fn auth_value(&self) -> String {
        format!("{}{}", self.app_id, self.service_token)
    }
}

#[cfg(test)]
mod tests {
    use super::{AppAuth, Auth};

    #[test]
    fn test_calculate_md5() {
        std::env::set_var("MD5_KEY_1", "DUMMY_KEY");
        let foo = TestStruct {
            foo: String::from("foo_bar_baz"),
            bar: 123456,
            auth: AppAuth {
                app_id: String::from("not-used"),
                auth: String::from("not-used"),
                service_token: String::from("not-used"),
            },
        };

        assert_eq!(foo.md5(), "f1575c382652f37a5504d52d3c8e5661");
    }

    #[test]
    fn test_auth() {
        let foo = TestStruct {
            foo: String::from("not used"),
            bar: 123456,
            auth: AppAuth {
                app_id: String::from("not-used"),
                auth: String::from("auth value"),
                service_token: String::from("not-used"),
            },
        };

        assert_eq!(foo.auth(), "auth value");
    }

    #[test]
    fn test_validate_auth() {
        std::env::set_var("MD5_KEY_1", "DUMMY_KEY");
        let foo = TestStruct {
            foo: String::from("foo_bar_baz"),
            bar: 123456,
            auth: AppAuth {
                app_id: String::from("not-used"),
                auth: String::from("f1575c382652f37a5504d52d3c8e5661"),
                service_token: String::from("not-used"),
            },
        };

        assert!(foo.validate_auth().is_ok());
    }
    #[test]
    fn test_validate_auth_old() {
        std::env::set_var("MD5_KEY_2", "DUMMY_KEY_2");
        let foo = TestStruct {
            foo: String::from("foo_bar_baz"),
            bar: 123456,
            auth: AppAuth {
                app_id: String::from("not-used"),
                auth: String::from("f43fe1da81f65514710dbe9481bb446b"),
                service_token: String::from("not-used"),
            },
        };

        assert!(foo.validate_auth_old().is_ok());
    }

    struct TestStruct {
        auth: AppAuth,
        foo: String,
        bar: i32,
    }

    impl Auth for TestStruct {
        fn auth(&self) -> &str {
            self.auth.auth.as_str()
        }

        fn service_token(&self) -> &str {
            "NOT_APPLICABLE"
        }

        fn auth_value(&self) -> String {
            format!("{}{}", self.foo, self.bar)
        }
    }
}
