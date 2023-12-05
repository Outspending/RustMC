#[macro_export]
macro_rules! packet {
    ($id: literal, $name: ident {
        $( $field: ident : $ty: ty ),* $(,)?
    }) => {
        use crate::Packet;
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Serialize, Deserialize)]
        pub struct $name {
            $( pub $field: $ty ),*
        }

        impl Packet for $name {

            fn id(&self) -> u8 {
                $id
            }

            fn serialize(&self) -> Vec<u8> {
                bincode::serialize(self).expect("Failed to serialize packet")
            }

            fn deserialize(data: Vec<u8>) -> Option<Self> {
                bincode::deserialize::<$name>(&data).ok()
            }

        }
    };
}
