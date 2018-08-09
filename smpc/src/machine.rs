extern crate futures;

use std::collections::HashMap;
use std::sync::mpsc;

use super::test_field::TestField;
use super::shamir;

enum Error {
    SplitError,
    JoinError,
}

trait Access {
    type Object : ThresholdSharable;
    type Address;
    type WriteResult;

    fn read(&self, Self::Address) -> Self::Object;
    fn write(&self, Self::Object, Self::Address) -> Self::WriteResult;
}

trait ThresholdSharable: Sized {
    type ID;
    
    fn split(Self, usize, usize) -> Result<Vec<(Self::ID, Self)>, Error>;
    fn join(&[(Self::ID, Self)], usize) -> Result<Self, Error>;
}

struct Memory {
    id: String,
    contents: HashMap<String, TestField>,
    inbound: mpsc::Receiver<(String, TestField)>,
    outgoing: HashMap<String, mpsc::Sender<(String, Option<TestField>)>>,
}

struct MemoryProxy {}

struct PVM {
    id: String,
    memory: Memory,
    memory_proxy: MemoryProxy,
}

impl Access for Memory {
    type Object = TestField;
    type Address = String;
    type WriteResult = Result<(), Error>;

    fn read(&self, addr: Self::Address) -> Self::Object {
        self.outgoing[addr].send((self.id, None))
    }
}

impl ThresholdSharable for TestField {
    type ID = Self;

    fn split(secret: Self, k: usize, n: usize) -> Result<Vec<(Self::ID, Self)>, Error> {
        match shamir::split(secret, k, n) {
            Ok(shares) => Ok(shares),
            Err(_) => Err(Error::SplitError),
        }
    }

    fn join(shares: &[(Self::ID, Self)], k: usize) -> Result<Self, Error> {
        match shamir::join(shares, k) {
            Ok(secret) => Ok(secret),
            Err(_) => Err(Error::JoinError),
        }
    }
}