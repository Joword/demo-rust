use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub enum ResultCode{
    SUCCESS = 1000,
    SERVICE_EXCEPTION = 1100,
    
}