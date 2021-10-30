

#[macro_export]
macro_rules! as_item { ($i:item) => {$i} }

#[macro_export]
macro_rules! gen_attr {
   ($name:tt; $($values:tt),+) =>{
         as_item! {
             #[derive(Debug, IntoAttribute)]
             enum $name { $($values),+ }
         }
   };
}