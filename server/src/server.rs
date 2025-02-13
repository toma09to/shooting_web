use std::collections::{HashMap, HashSet};

use actix::prelude::*;
use rand::{rngs::ThreadRng, Rng};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);
