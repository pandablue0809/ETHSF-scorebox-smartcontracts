use near_sdk::{
    collections::{LookupMap, Vector},
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
    PanicOnDefault, BorshStorageKey, AccountId, 
    log, env, near_bindgen, Balance, Promise
};


// user's score - onchain
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct UserOn {
    pub score: u16,
    pub description: Vec<u8>,
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct UserOff {
    pub score: u16,
    pub description: String,
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    Accounts { account_hash: Vec<u8> }
}

// Define the contract structure
#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    owner_id: AccountId,
    records: LookupMap<String, Vector<UserOn>>
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ScoreVec {
    scores: Vec<UserOff>
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init] // implies you MUST call init() to initialize contract
    pub fn init(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            records: LookupMap::new(b"m"),
        }
    }

    #[payable]
    pub fn upload_score(
        &mut self,
        score: u16,
        description: String,
        beneficiary: AccountId,
        amount: Balance
    ) -> bool {
        //transfer an 'amount' of NEARs to an account id
        Promise::new(beneficiary).transfer(amount);

        let account_id: String = String::from(env::signer_account_id());
        let novel = UserOn {
            score: score,
            description: description.as_bytes().to_vec(),
        };

        let pedigree = self.records.get(&account_id);
        match pedigree {
            None => {
                log!{"New user uploaded a score"};
                let mut v = Vector::new(
                    StorageKey::Accounts {account_hash: env::sha256(account_id.as_bytes()) }
                );
                v.push(&novel);
                // update the score count iff you succeeded writing it to blockchain`
                self.records.insert(&account_id, &v);
                log!("Score live on NEAR!");
            }
            Some(v) => {
                log!("A returning user uploaded a score");
                let mut w = v;
                w.push(&novel);
                self.records.insert(&account_id, &w);
                log!("Score live on NEAR!");
            }
        }
        let outcome: bool = true;
        return outcome
    }


    // this is a view method
    pub fn get_scores(&self, account_id: String) -> ScoreVec  {

        let pedigree = self.records.get(&account_id);
        match pedigree {
            None => {env::panic_str("No credit scores for this user");}
            Some(z) => {
                let mut record = vec![];
                for x in z.iter() {
                    let entry = UserOff {
                        score: x.score,
                        description: String::from_utf8(x.description).unwrap(),
                    };
                    record.push(entry);
                };
                return ScoreVec {
                    scores: record
                }
            }
        }
    }
}
