//! Fork from Counter Tutorial
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

// add the following attributes to prepare your code for serialization and invocation on the blockchain
// More built-in Rust attributes here: https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    // See more data types at https://doc.rust-lang.org/book/ch03-02-data-types.html
    result: i8, // i8 is signed. unsigned integers are also available: u8, u16, u32, u64, u128
}

#[near_bindgen]
impl Counter {
    pub fn get_result(&self) -> i8 {
        return self.result;
    }
    pub fn mul(&mut self, x: i8, y: i8) {
        self.result = i8::wrapping_mul(x, y);

        let log_message = format!("Multiply result {}", self.result);
        env::log(log_message.as_bytes());
        after_counter_change();
    }

    pub fn div(&mut self, x: i8, y: i8) {
        // self.result -= 1;
        self.result = i8::wrapping_div(x, y);
        let log_message = format!("Divide retult {}", self.result);
        env::log(log_message.as_bytes());
        after_counter_change();
    }

    /// Reset to zero.
    pub fn reset(&mut self) {
        self.result = 0;
        // Another way to log is to cast a string into bytes, hence "b" below:
        env::log(b"Reset result to zero");
    }
}

// unlike the struct's functions above, this function cannot use attributes #[derive(…)] or #[near_bindgen]
// any attempts will throw helpful warnings upon 'cargo build'
// while this function cannot be invoked directly on the blockchain, it can be called from an invoked function
fn after_counter_change() {
    // show helpful warning that i8 (8-bit signed integer) will overflow above 127 or below -128
    env::log("Done!".as_bytes());
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-counter-tutorial-1 -- --nocapture
 * Note: 'rust-counter-tutorial' comes from cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::json_types::ValidAccountId;
    use near_sdk::serde::export::TryFrom;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;
    use near_sdk::MockedBlockchain;

    // simple helper function to take a string literal and return a ValidAccountId
    fn to_valid_account(account: &str) -> ValidAccountId {
        ValidAccountId::try_from(account.to_string()).expect("Invalid account")
    }

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // mark individual unit tests with #[test] for them to be registered and fired
    #[test]
    fn multiply() {
        // set up the mock context into the testing environment
        let context = get_context(to_valid_account("foo.near"));
        testing_env!(context.build());
        // instantiate a contract variable with the counter at zero
        let mut contract = Counter { result: 0 };
        contract.mul(5, 20);
        println!("Result after multiply: {}", contract.get_result());
        // confirm that we received 1 when calling get_result
        assert_eq!(100, contract.get_result());
    }

    #[test]
    fn divide() {
        let context = VMContextBuilder::new();
        testing_env!(context.build());
        let mut contract = Counter { result: 0 };
        contract.div(20, 10);
        println!("Result after divide: {}", contract.get_result());
        // confirm that we received -1 when calling get_result
        assert_eq!(2, contract.get_result());
    }

    #[test]
    fn divide_and_reset() {
        let context = VMContextBuilder::new();
        testing_env!(context.build());
        let mut contract = Counter { result: 0 };
        contract.div(10, 5);
        contract.reset();
        println!("Result after reset: {}", contract.get_result());
        // confirm that we received -1 when calling get_result
        assert_eq!(0, contract.get_result());
    }
}
