use crate::{ProofWithPublicInputs, PublicKey, Signature, C, D, F};

use crate::path_voucher::PathVoucher;

pub(crate) trait Voucher {
    fn incremental_vouch(
        existing_voucher: impl Voucher,
        origin: PublicKey,
        locus: PublicKey,
        signature: Signature,
        input_degree: F,
    ) -> PathVoucher;
    fn degree(&self) -> F;
    fn is_origin(&self) -> bool;
    fn proof_data(&self) -> &ProofWithPublicInputs<F, C, D>;
    fn verify(&self) -> bool;
    fn origin(&self) -> PublicKey;
}
