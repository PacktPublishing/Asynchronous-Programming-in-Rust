use std::collections::HashMap;

use crate::future::Future;

pub struct Executor {
    tasks: HashMap<usize, dyn Future<Output = String>>,
}