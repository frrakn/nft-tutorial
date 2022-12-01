use crate::{nft_core::NonFungibleTokenCore, *};

#[near_bindgen]
impl Contract {
    //Query for the total supply of NFTs on the contract
    pub fn nft_total_supply(&self) -> U128 {
        U128(self.token_metadata_by_id.len().into())
    }

    //Query for nft tokens on the contract regardless of the owner using pagination
    pub fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonToken> {
        let start: u128 = u128::from(from_index.unwrap_or(U128(0)));

        self.token_metadata_by_id
            .iter()
            .skip(start as usize)
            .take(limit.unwrap_or(50) as usize)
            .map(|(token_id, _)| self.nft_token(token_id.clone()).unwrap())
            .collect()
    }

    //get the total supply of NFTs for a given owner
    pub fn nft_supply_for_owner(&self, account_id: AccountId) -> U128 {
        if let Some(token_set) = self.tokens_per_owner.get(&account_id) {
            U128(u128::from(token_set.len()))
        } else {
            U128(0)
        }
    }

    //Query for all the tokens for an owner
    pub fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<JsonToken> {
        if let Some(token_set) = self.tokens_per_owner.get(&account_id) {
            let start = u128::from(from_index.unwrap_or(U128(0)));

            token_set
                .iter()
                .skip(start as usize)
                .take(limit.unwrap_or(50) as usize)
                .map(|token_id| self.nft_token(token_id.clone()).unwrap())
                .collect()
        } else {
            vec![]
        }
    }
}
