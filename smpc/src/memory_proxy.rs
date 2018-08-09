extern crate futures;

use memory_proxy::futures::future::Future;

enum MemoryAction {
    Load,
    Store,
}

enum RequestError {
    InsufficientPermissions,
    RequestFailed,
}

trait AccessRequest {
    type ID : PartialEq;
    type Address : PartialEq;

    fn identity(&self) -> Self::ID;
    fn address(&self) -> Self::Address;
    fn request_type(&self) -> MemoryAction;
}

trait MemoryProxy<T, U>
    where T: AccessRequest, U: Future {

    fn load(&mut self, T::Address) -> U;
    fn store(&mut self, T::Address) -> U;
    fn has_permission(identity: T::ID, request_type: MemoryAction) -> bool;

    fn perform_request(&mut self, request: T) -> U {
        match request.request_type() {
            Load => self.load(request.address()),
            Store => self.store(request.address()),
        }
    }
}