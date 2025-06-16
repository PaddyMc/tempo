use crate::context::MalachiteContext;
use malachite_core_types::{PrivateKey, PublicKey, Signature, SigningProvider, SigningScheme};
use std::fmt;

// TODO: Implement Ed25519Provider
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ed25519Provider {
    private_key: [u8; 32],
}

impl Ed25519Provider {
    pub fn new() -> Self {
        Self {
            private_key: [0u8; 32], // Default/placeholder key
        }
    }

    pub fn with_private_key(private_key: [u8; 32]) -> Self {
        Self { private_key }
    }

    pub fn private_key(&self) -> &[u8; 32] {
        &self.private_key
    }
}

impl Default for Ed25519Provider {
    fn default() -> Self {
        Self::new()
    }
}

impl SigningScheme for Ed25519Provider {
    type PrivateKey = [u8; 32];
    type PublicKey = Vec<u8>;
    type Signature = Vec<u8>;
    type DecodingError = std::io::Error;

    fn decode_signature(bytes: &[u8]) -> Result<Self::Signature, Self::DecodingError> {
        Ok(bytes.to_vec())
    }

    fn encode_signature(signature: &Self::Signature) -> Vec<u8> {
        signature.clone()
    }
}

// impl Ed25519Provider {
//     pub fn new(private_key: PrivateKey) -> Self {
//         Self { private_key }
//     }

//     pub fn private_key(&self) -> &PrivateKey {
//         &self.private_key
//     }

//     pub fn sign(&self, data: &[u8]) -> Signature {
//         self.private_key.sign(data)
//     }

//     pub fn verify(&self, data: &[u8], signature: &Signature, public_key: &PublicKey) -> bool {
//         public_key.verify(data, signature).is_ok()
//     }
// }

// impl SigningProvider<MalachiteContext> for Ed25519Provider {
//     fn sign_vote(&self, vote: Vote) -> SignedVote<TestContext> {
//         let signature = self.sign(&vote.to_sign_bytes());
//         SignedVote::new(vote, signature)
//     }

//     fn verify_signed_vote(
//         &self,
//         vote: &Vote,
//         signature: &Signature,
//         public_key: &PublicKey,
//     ) -> bool {
//         public_key.verify(&vote.to_sign_bytes(), signature).is_ok()
//     }

//     fn sign_proposal(&self, proposal: Proposal) -> SignedProposal<TestContext> {
//         let signature = self.private_key.sign(&proposal.to_sign_bytes());
//         SignedProposal::new(proposal, signature)
//     }

//     fn verify_signed_proposal(
//         &self,
//         proposal: &Proposal,
//         signature: &Signature,
//         public_key: &PublicKey,
//     ) -> bool {
//         public_key
//             .verify(&proposal.to_sign_bytes(), signature)
//             .is_ok()
//     }

//     fn sign_proposal_part(&self, proposal_part: ProposalPart) -> SignedProposalPart<TestContext> {
//         let signature = self.private_key.sign(&proposal_part.to_sign_bytes());
//         SignedProposalPart::new(proposal_part, signature)
//     }

//     fn verify_signed_proposal_part(
//         &self,
//         proposal_part: &ProposalPart,
//         signature: &Signature,
//         public_key: &PublicKey,
//     ) -> bool {
//         public_key
//             .verify(&proposal_part.to_sign_bytes(), signature)
//             .is_ok()
//     }

//     fn sign_vote_extension(&self, extension: Bytes) -> SignedExtension<TestContext> {
//         let signature = self.private_key.sign(extension.as_ref());
//         malachitebft_core_types::SignedMessage::new(extension, signature)
//     }

//     fn verify_signed_vote_extension(
//         &self,
//         extension: &Bytes,
//         signature: &Signature,
//         public_key: &PublicKey,
//     ) -> bool {
//         public_key.verify(extension.as_ref(), signature).is_ok()
//     }
// }
