use alloy_consensus::Header;
use reth_ethereum_primitives::BlockBody;
use reth_primitives_traits::{Block, InMemorySize};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct MalachiteBlock {
    pub header: Header,
    pub body: MalachiteBlockBody,
}

impl InMemorySize for MalachiteBlock {
    fn size(&self) -> usize {
        self.header.size() + self.body.inner.size()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MalachiteBlockBody {
    #[serde(flatten)]
    pub inner: BlockBody,
}

// TODO: Do we need a custom MalachiteBlock?
// impl Block for MalachiteBlock {
//     type Header = Header;
//     type Body = MalachiteBlockBody;

//     fn new(header: Self::Header, body: Self::Body) -> Self {
//         Self { header, body }
//     }

//     fn header(&self) -> &Self::Header {
//         &self.header
//     }

//     fn body(&self) -> &Self::Body {
//         &self.body
//     }

//     fn split(self) -> (Self::Header, Self::Body) {
//         (self.header, self.body)
//     }

//     fn rlp_length(header: &Self::Header, body: &Self::Body) -> usize {
//         todo!()
//     }
// }
