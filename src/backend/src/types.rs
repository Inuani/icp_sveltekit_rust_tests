
use candid::{Decode, Encode, Principal, CandidType, Deserialize};
use ic_stable_structures::{ storable::Bound, DefaultMemoryImpl, StableBTreeMap, Storable };
use std::{borrow::Cow, cell::RefCell};

pub type Blob = Vec<u8>;
const MAX_VALUE_SIZE: u32 = 1000;
// u32 is 32 bits and 4 bytes
// for struct you combine all the fields

#[derive(Clone, CandidType, Deserialize)]
pub struct Denizen {
    pub principal: Principal,
    pub dname: String,
	pub firstname: Option<String>,
	// pub lastname: String,
	// pub email: String,
	// pub birthdate: String,
	pub xp: u64,
	pub level: u64,
	pub token_balance: u64,
	// pub achievements: Vec<Achievement>,
	// pub role: u8,
	// pub pfp: Blob,
}

#[derive(Default, Clone, CandidType, Deserialize, Debug)]
pub struct  Achievement {
	pub icon: Blob,
	pub name: String,
	pub description: String,
	pub xp: u64,
	pub token: u64,
	pub ammount: Option<u64>,
	pub awarded_ammount: u64,
}

#[derive(Clone, CandidType, Deserialize, Ord, Eq, PartialOrd, PartialEq)]
pub struct StablePrincipal(pub Principal);

impl StablePrincipal {
    pub fn into_inner(self) -> Principal {
        self.0
    }
}

impl Storable for StablePrincipal {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
  }

impl Storable for Denizen {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}