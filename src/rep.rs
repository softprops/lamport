use rustc_serialize::{Decodable, json};

#[derive(Debug, RustcDecodable)]
pub struct Context {
    pub awsRequestId: String,
    pub functionName: String,
    pub functionVersion: String,
    pub invokeid: String,
    pub isDefaultFunctionVersion: bool,
    pub logGroupName: String,
    pub logStreamName: String,
    pub memoryLimitInMB: String
}

#[derive(Debug, RustcDecodable)]
pub struct Payload<D: Decodable> {
    pub event: D,
    pub context: Context
}
