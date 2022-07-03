use crate::mc::Identifiable;
use crate::prelude::{ItemStack, Material};

pub trait CommandLike {
    fn compile(&mut self) -> String
    where
        Self: Sized;
}

impl CommandLike for ItemStack {
    fn compile(&mut self) -> String
    where
        Self: Sized,
    {
        self.stringified()
    }
}

impl CommandLike for Material {
    fn compile(&mut self) -> String
    where
        Self: Sized,
    {
        self.id().to_string()
    }
}

impl<T> CommandLike for T
where
    T: Into<String> + Clone,
{
    fn compile(&mut self) -> String
    where
        Self: Sized,
    {
        Clone::clone(self).into()
    }
}

macro_rules! declare_commands {
    (
        $(
            $(generic [$gen_type:ident : $gen_bound:ident $([$_i_gen:ident])?])? command $command_name:literal $struct_name:ident($(
                $(opt $opt_type:ident $opt_name:ident)?
                $(req $def_type:ident $def_name:ident)?
            ),* $(,)*)
        );* $(;)*
    ) => {
        $(
            #[derive(Debug, Clone)]
            pub struct $struct_name $(<$gen_type>)? {
                $(
                $($opt_name: Option<Option<$opt_type>>,)?
                $($def_name: Option<$def_type>,)?
                )*
            }

            impl $(<$gen_type>)? CommandLike for $struct_name $(<$gen_type>)? where $($gen_type: $gen_bound $(<$_i_gen>)? + Clone)? {
                fn compile(&mut self) -> String {
                    let mut buf = String::new();
                    buf.push_str($command_name);
                    $(
                        $(
                            if let Some(s) = self.$opt_name {
                                if let Some(mut d) = s {
                                    buf.push_str(format!(" {}", d.compile()).as_str());
                                };
                            };
                        )?
                        $(
                            let mut $def_name = Clone::clone(&self.$def_name).unwrap();
                            buf.push_str(format!(" {}", $def_name.compile()).as_str());
                        )?
                    )*
                    buf
                }
            }

            impl $(<$gen_type>)? $struct_name $(<$gen_type>)? where $($gen_type: $gen_bound $(<$_i_gen>)? + Clone)? {
                pub fn builder() -> Self {
                    Self {
                        $(
                            $($opt_name: None,)?
                            $($def_name: None,)?
                        )*
                    }
                }

                pub fn new($($($opt_name: Option<$opt_type>,)? $($def_name: $def_type,)?)*) -> Self where Self: Sized {
                    Self {
                        $(
                            $($opt_name: Some($opt_name),)?
                            $($def_name: Some($def_name),)?
                        )*
                    }
                }

                $(
                    $(
                        pub fn $opt_name(&mut self, value: Option<$opt_type>) -> Self where Self: Sized {
                            self.$opt_name = Some(value);
                            self.clone()
                        }
                    )?

                    $(
                        pub fn $def_name(&mut self, value: $def_type) -> Self where Self: Sized {
                            self.$def_name = Some(value);
                            self.clone()
                        }
                    )?
                )*
            }
        )*
    }
}

declare_commands! {
    generic[T: Into[String]] command "give" GiveCommand(
        req T selector,
        req ItemStack item
    );
}
