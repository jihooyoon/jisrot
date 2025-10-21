use std::borrow::Cow;

use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Getters, Clone, Deserialize, Serialize, PartialEq)]
#[getset(get = "pub")]
pub struct UiOption {
    pub value: Cow<'static, str>,
    pub text: Cow<'static, str>,
    pub connected_data: Option<Cow<'static, str>>,
}
