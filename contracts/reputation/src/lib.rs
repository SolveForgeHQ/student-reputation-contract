#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env, String, Vec,
};

#[contracttype]
#[derive(Clone)]
pub struct Credential {
    pub skill: String,
    pub verified_by: Address,
    pub timestamp: u64,
    pub endorsements: u32,
}

#[contracttype]
pub enum DataKey {
    Credential(Address, String),
    ReputationScore(Address),
}

#[contract]
pub struct ReputationContract;

#[contractimpl]
impl ReputationContract {

    pub fn add_credential(
        env: Env,
        student: Address,
        skill: String,
        verified_by: Address,
    ) {
        verified_by.require_auth();
        let credential = Credential {
            skill: skill.clone(),
            verified_by,
            timestamp: env.ledger().timestamp(),
            endorsements: 0,
        };
        env.storage()
            .persistent()
            .set(&DataKey::Credential(student, skill), &credential);
    }

    pub fn endorse_skill(
        env: Env,
        student: Address,
        skill: String,
        endorser: Address,
    ) {
        endorser.require_auth();
        let key = DataKey::Credential(student, skill);
        if let Some(mut credential) = env
            .storage()
            .persistent()
            .get::<DataKey, Credential>(&key)
        {
            credential.endorsements += 1;
            env.storage().persistent().set(&key, &credential);
        }
    }

    pub fn verify_credential(
        env: Env,
        student: Address,
        skill: String,
    ) -> bool {
        env.storage()
            .persistent()
            .has(&DataKey::Credential(student, skill))
    }

    pub fn get_reputation_score(
        env: Env,
        student: Address,
        skills: Vec<String>,
    ) -> u32 {
        let mut score: u32 = 0;
        for skill in skills.iter() {
            if let Some(credential) = env
                .storage()
                .persistent()
                .get::<DataKey, Credential>(
                    &DataKey::Credential(student.clone(), skill)
                )
            {
                score += 10;
                score += credential.endorsements * 5;
            }
        }
        score
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_add_and_verify_credential() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, ReputationContract);
        let client = ReputationContractClient::new(&env, &contract_id);

        let student = Address::generate(&env);
        let verifier = Address::generate(&env);
        let skill = String::from_str(&env, "React");

        client.add_credential(&student, &skill, &verifier);
        assert!(client.verify_credential(&student, &skill));
    }

    #[test]
    fn test_endorse_skill() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, ReputationContract);
        let client = ReputationContractClient::new(&env, &contract_id);

        let student = Address::generate(&env);
        let verifier = Address::generate(&env);
        let endorser = Address::generate(&env);
        let skill = String::from_str(&env, "TypeScript");

        client.add_credential(&student, &skill, &verifier);
        client.endorse_skill(&student, &skill, &endorser);

        let credential = client.get_reputation_score(
            &student,
            &Vec::from_array(&env, [skill])
        );
        assert!(credential > 10);
    }
}
