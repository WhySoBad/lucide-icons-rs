use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct IconInfo {
    encoded_code: String,
    prefix: String,
    class_name: String,
    unicode: String,
}

impl IconInfo {
    pub fn unicode(&self) -> char {
        let bytes = u16::from_str_radix(&self.encoded_code.as_str()[1..], 16)
            .expect("should parse icon unicode as u16");
        char::from_u32(bytes as u32).expect("should be a vaild unicode character")
    }
}
