#[macro_export]
macro_rules! as_item { ($i:item) => {$i} }

#[macro_export]
macro_rules! generate_attr {
    (enum $name:tt ; $($values:tt),+) =>{
         as_item! {
             #[derive(Debug,PartialEq, IntoAttribute)]
             pub enum $name { $($values),+ }
         }
         impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.write_str(format!("{:?}",self).as_str())
            }
         }
   };
    (enum $name:tt for $($owners:tt),+; $($values:tt),+) =>{
         as_item! {
             #[derive(Debug,PartialEq, IntoAttribute)]
             pub enum $name { $($values),+ }
         }

         $(impl $owners {
                 pub fn $name(elem:$name) -> Attribute {
                      elem.into_attr()
                  }
             }
         )+

   };
    (enum $name:tt for $($owners:tt),+; $($values:tt),+;$default:tt ) =>{
         as_item! {
             #[derive(Debug,PartialEq, IntoAttribute)]
             pub enum $name { $($values),+ }
         }

         impl Default for $name{
             fn default() -> Self { $name::$default }
         }

         $(impl $owners{
                 pub fn $name(elem:$name) -> Attribute {
                      elem.into_attr()
                  }
             }
         )+


   };
    (struct $name:tt for $($owners:tt),+; $ty:tt) =>{
        as_item! {
             #[derive(Debug,PartialEq, IntoAttribute)]
             pub struct $name ($ty);
        }
        $(impl $owners {
                 pub fn $name(elem:$ty) -> Attribute {
                     $name(elem).into_attr()
                  }
             }
        )+

    };
    (struct $name:tt for $($owners:tt),+; $ty:tt; $default:expr) =>{
         as_item! {
             #[derive(Debug,PartialEq, IntoAttribute)]
             pub struct $name ($ty);
         }
        impl Default for $name{
                 fn default() -> Self { $name($default) }
        }
         $(
             impl $owners{
                 pub fn $name(elem:$ty) -> Attribute {
                     $name(elem).into_attr()
                  }
             }
         )+

   };
}