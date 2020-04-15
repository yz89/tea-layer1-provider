#[macro_use]
extern crate wascc_codec as codec;

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

mod layer1;

use crate::layer1::Layer1;

use codec::capabilities::{CapabilityProvider, Dispatcher, NullDispatcher};
use codec::core::OP_BIND_ACTOR;
use wascc_codec::core::CapabilityConfiguration;
use codec::{serialize, deserialize};
use std::error::Error;
use std::sync::RwLock;

const SYSTEM_ACTOR: &str = "system";

//#[cfg(not(feature = "static_plugin"))]
capability_provider!(Layer1Provider, Layer1Provider::new);

const CAPABILITY_ID: &str = "tea:layer1";

pub struct Layer1Provider {
    dispatcher: RwLock<Box<dyn Dispatcher>>,
    layer1: RwLock<Layer1>,
}

impl Default for Layer1Provider {
    fn default() -> Self {
        let _ = env_logger::try_init();

        Layer1Provider {
            dispatcher: RwLock::new(Box::new(NullDispatcher::new())),
            layer1: RwLock::new(Layer1::new()),
        }
    }
}

impl Layer1Provider {
    pub fn new() -> Self {
        Self::default()
    }

    fn configure(
        &self,
        _config: CapabilityConfiguration,
    ) -> Result<Vec<u8>, Box<dyn Error>> {

        //TODO: Config here

        Ok(vec![])
    }
}

impl Layer1Provider {
    pub fn tpm_info(&self, actor: &str, req: Layer1Message) -> Result<Vec<u8>, Box<dyn Error>> {
        println!("Layer1Provider actor:{} called tpm_info.", actor);
        let layer1 = self.layer1.read().unwrap();
        let res = layer1.tpm_info(actor, req.key).unwrap();
        println!("return from layer1.tpm_info: {:?}", res);
        Ok(serialize(res)?)
    }
}

impl CapabilityProvider for Layer1Provider {
    fn capability_id(&self) -> &'static str {
        CAPABILITY_ID
    }

    // Invoked by the runtime host to give this provider plugin the ability to communicate
    // with actors
    fn configure_dispatch(&self, dispatcher: Box<dyn Dispatcher>) -> Result<(), Box<dyn Error>> {
        trace!("Dispatcher received.");
        let mut lock = self.dispatcher.write().unwrap();
        *lock = dispatcher;

        Ok(())
    }

    fn name(&self) -> &'static str {
        "TEA project: Layer One Provider"
    }


    // Invoked by host runtime to allow an actor to make use of the capability
    // All providers MUST handle the "configure" message, even if no work will be done
    fn handle_call(&self, actor: &str, op: &str, msg: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        trace!("Received host call from {}, operation - {}", actor, op);

        match op {
            OP_BIND_ACTOR if actor == SYSTEM_ACTOR => self.configure(deserialize(msg)?),
            layer1::OP_TPM_INFO => {
                self.tpm_info(actor, deserialize(msg)?)
            }
            _ => Err("bad dispatch".into()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer1Message {
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer1Reply {
    pub value: i32,
}