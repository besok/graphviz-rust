use dot_structures::Attribute;
pub trait IntoAttribute{
    fn into_attr(self) -> Attribute;
}
