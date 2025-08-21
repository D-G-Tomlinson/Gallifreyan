
use std::convert::TryFrom;
pub struct Svg(String);
impl Svg {
    pub fn svg(self) -> String { self.0 }
}
impl TryFrom<String> for Svg {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let input = value.trim().to_lowercase().chars().collect::<Vec<char>>();
        if input.into_iter().any(|c| !c.is_ascii_lowercase()) {
            return Err("only letters for now");
        }
        return Err("haven't implemented");
    }
}
pub fn get_image(text: &str) -> String {
    return match Svg::try_from(text.to_string()) {
        Ok(image) => image.svg(),
        Err(error) => format!("<p>{}</p>",error.to_string())
    }
}