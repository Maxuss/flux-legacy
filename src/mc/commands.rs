use crate::mc::{Identifiable, Identifier};
use crate::prelude::{ItemStack, Material};

pub trait CommandLike {
    fn compile(&mut self) -> String;
}

impl CommandLike for ItemStack {
    fn compile(&mut self) -> String {
        self.stringified()
    }
}

impl CommandLike for Material {
    fn compile(&mut self) -> String {
        self.id().to_string()
    }
}

impl CommandLike for Identifier {
    fn compile(&mut self) -> String {
        self.to_string()
    }
}

macro_rules! prim_impl {
    ($($typ:ident),* $(,)*) => {
        $(
        impl CommandLike for $typ {
            fn compile(&mut self) -> String {
                self.to_string()
            }
        }
        )*
    }
}

prim_impl!(String, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

macro_rules! declare_commands {
    (
        $(
            command $command_name:literal $struct_name:ident($(
                $(opt $opt_type:ident $opt_name:ident)?
                $(req $def_type:ident $def_name:ident)?
            ),* $(,)*)
        );* $(;)*
    ) => {
        $(
            #[derive(Debug, Clone)]
            pub struct $struct_name {
                $(
                $($opt_name: Option<Option<$opt_type>>,)?
                $($def_name: Option<$def_type>,)?
                )*
            }

            impl CommandLike for $struct_name {
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
                            let mut $def_name = self.$def_name.to_owned().unwrap();
                            buf.push_str(format!(" {}", $def_name.compile().as_str()).as_str());
                        )?
                    )*
                    buf
                }
            }

            impl $struct_name {
                pub fn builder() -> Self {
                    Self {
                        $(
                            $($opt_name: None,)?
                            $($def_name: None,)?
                        )*
                    }
                }

                pub fn new($($($opt_name: Option<$opt_type>,)? $($def_name: $def_type,)?)*) -> Self {
                    Self {
                        $(
                            $($opt_name: Some($opt_name),)?
                            $($def_name: Some($def_name),)?
                        )*
                    }
                }

                $(
                    $(
                        pub fn $opt_name(&mut self, value: Option<$opt_type>) -> Self {
                            self.$opt_name = Some(value);
                            self.clone()
                        }
                    )?

                    $(
                        pub fn $def_name(&mut self, value: $def_type) -> Self {
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
    command "give" GiveCommand(
        req ItemStack item,
        opt i8 amount
    );
}
