
_Here's the original bounty (by Antoine)_
**Bounty adress**: *will be updated soon* |
--- |
 **Task**: Write a useful tutorial |
 **Description**: Teach something valuable |
 **Devcash reward**: 100, 000 {D} |

### My Tutorial
:warning: _work in progress_

A beginners guide for building with [Substrate](https://substrate.dev/).

The aim of this tutorial is to: (1) provide a brief overview of Subtrate's building blocks and key notions and (2) provide a guide for adding custom functionality to a template node.

It will cover: 
- Basic setup and tools 
- A step-by-step to launch a Substrate chain with a collection of pallets (also known as modules) that provide functionality for the runtime to deploy and execute WebAssembly smart-contracts.
---

# 1. Getting things setup :computer:
## Installing the Substrate node template
Make sure you're environment is all setup for using Rust. Refer to [this tutorial](https://substrate.dev/docs/en/knowledgebase/getting-started/) if it isn't already.

In this first step, we're going to clone the node template and compile it. This can take a little while depending on your hardware, don't panic (took me 25m57s on a fairly powerful laptop).

i. Clone the Node Template
```bash 
git clone -b v2.0.0 --depth 1 https://github.com/substrate-developer-hub/substrate-node-template
```

ii. Initialize WASM
```bash
make init
```

iii. Compile it
```bash 
make build
```
:clock12: :clock1230: :clock1:

While it compiles, let's understand what this template is and what it will do for us. :flashlight:

The template is basically boiler plate from which you can customize your runtime. Referred to as a _"FRAME-based Substrate node"_, "FRAME" is short for: _a Framework for Runtime Aggregation of Modularized Entities_. Its essentially a pretty mightly library of libraries for building things with Substrate. :muscle:

In what we're compiling, our runtime has the following pallets configured (see [docs](https://github.com/substrate-developer-hub/substrate-node-template/blob/cbf28f104237648512e8e3b6af2126e840d56243/runtime/src/lib.rs#L269-L287)). Take a look at the  FRAME documentation to better understand what each one is doing: 

- [``frame_system``](https://crates.parity.io/frame_system/index.html) - provides low-level access to core types and cross-cutting utilities.
- [``pallet_randomness_collective_flip``](https://crates.parity.io/pallet_randomness_collective_flip/index.html) - provides a ``random`` function that generates low-influence random values based on the block hashes from the previous 81 blocks.
- [``pallet_timestamp``](https://crates.parity.io/pallet_timestamp/index.html) - provides functionality to get and set the on-chain time.
- [``pallet_aura``](https://crates.parity.io/pallet_aura/index.html) - extends Aura consensus by managing offline reporting.
- [``pallet_grandpa``](https://crates.parity.io/pallet_grandpa/index.html) - GRANDPA Consensus module for runtime. This manages the GRANDPA authority set ready for the native code. 
- [``pallet_balances``](https://crates.parity.io/pallet_balances/index.html) - provides functionality for handling accounts and balances.
- [``pallet_transaction_payment``](https://crates.parity.io/pallet_transaction_payment/index.html) - provides the basic logic needed to pay the absolute minimum amount needed for a transaction to be included. 
- [``pallet_sudo``](https://crates.parity.io/pallet_sudo/index.html) - allows for a single account (called the "sudo key") to execute dispatchable functions that require a Root call or designate a new account to replace them as the sudo key. 
- [``pallet_template``](https://crates.parity.io/pallet_template/index.html) - _doesn't do anything, just there to be customized_

:point_right: Learn more by checking out the Substrate Developer Hub [docs](https://substrate.dev/docs/en/).

## Explore the template setup
Below is the node templates directory tree. Get familiar with the structure so you know where to find the files we'll be modifying in this tutorial. :palm_tree:

```bash
├── Cargo.lock
├── Cargo.toml
├── docker-compose.yml
├── LICENSE
├── Makefile
├── node
│   ├── build.rs                    
│   ├── Cargo.toml   <------ low-level dependencies
│   └── src
│       ├── chain_spec.rs   <------ this is where you can setup initial configs for your chain   
│       ├── cli.rs
│       ├── command.rs
│       ├── lib.rs
│       ├── main.rs
│       ├── rpc.rs
│       └── service.rs
├── pallets          <------ here's where you put new pallets you create
│   └── template
│       ├── Cargo.toml
│       └── src
│           ├── lib.rs
│           ├── mock.rs
│           └── tests.rs
├── README.md
├── runtime
│   ├── build.rs
│   ├── Cargo.toml   <------ you'll need to update things here for every new pallet you include in your runtime
│   └── src
│       └── lib.rs   <------  here's where you specify your runtimes configuration 
├── scripts
│   ├── docker_run.sh
│   └── init.sh
└── target          <------ where the runtime compiles to
    └── release
        ├── build
```
## Let's get back to it :hourglass:

# 2. Launching your node

Assuming everything has compiled, it's time to test our node. Running this command will launch it in development mode:
``./target/release/node-template --dev --tmp``

If all is working fine, you should see blocks being created in your terminal.

To see what's happening with a more slick UI, head to: https://polkadot.js.org/apps/ and switch to Local Node. 

# 3. Importing a new pallet
We'll be importing a number of dependencies to implement the [``pallets_contracts``](https://substrate.dev/rustdocs/v2.0.0/pallet_contracts/index.html) pallet. These will be:

```bash
pallet-contracts = { version = '2.0.0', default_features = false }
pallet-contracts-primitives = { version = '2.0.0', default_features = false }
pallet-contracts-rpc-runtime-api = { version = '0.8.0', default-features = false }
```

## Setting things up for our runtime :vertical_traffic_light:

**I. Update ``runtime/Cargo.toml`` by adding the following:**
```bash
[dependencies]
#--snip--
pallet-contracts = { version = '2.0.0', default_features = false }
pallet-contracts-primitives = { version = '2.0.0', default_features = false }
```
and: 

```bash
[features]
default = ["std"]
std = [
    #--snip--
    'pallet-contracts/std',
    'pallet-contracts-primitives/std',    
    #--snip--
]
```
Run the following command to check whether the pallet has been imported with no errors: 
```bash
SKIP_WASM_BUILD=1 cargo check -p node-template-runtime
```

## Setting up the Contracts pallet :memo:

**II. Configure the pallet:**

For the Contracts pallet, we can find out how to configure it by looking at its ``Trait`` types (see the [docs](https://substrate.dev/rustdocs/v2.0.0/pallet_contracts/trait.Trait.html)). In this step, we're going to configure ``pallet_contract`` by setting our ```const``` values. 

In ``runtime/src/lib.rs``, paste the following configurations for the Contracts Pallet:

```bash
// Contracts price units.
pub const MILLICENTS: Balance = 1_000_000_000;
pub const CENTS: Balance = 1_000 * MILLICENTS;
pub const DOLLARS: Balance = 100 * CENTS;

parameter_types! {
    pub const TombstoneDeposit: Balance = 16 * MILLICENTS;
    pub const RentByteFee: Balance = 4 * MILLICENTS;
    pub const RentDepositOffset: Balance = 1000 * MILLICENTS;
    pub const SurchargeReward: Balance = 150 * MILLICENTS;
}

impl pallet_contracts::Trait for Runtime {
    type Time = Timestamp;
    type Randomness = RandomnessCollectiveFlip;
    type Currency = Balances;
    type Event = Event;
    type DetermineContractAddress = pallet_contracts::SimpleAddressDeterminer<Runtime>;
    type TrieIdGenerator = pallet_contracts::TrieIdFromParentCounter<Runtime>;
    type RentPayment = ();
    type SignedClaimHandicap = pallet_contracts::DefaultSignedClaimHandicap;
    type TombstoneDeposit = TombstoneDeposit;
    type StorageSizeOffset = pallet_contracts::DefaultStorageSizeOffset;
    type RentByteFee = RentByteFee;
    type RentDepositOffset = RentDepositOffset;
    type SurchargeReward = SurchargeReward;
    type MaxDepth = pallet_contracts::DefaultMaxDepth;
    type MaxValueSize = pallet_contracts::DefaultMaxValueSize;
    type WeightPrice = pallet_transaction_payment::Module<Self>;
}

```

:heavy_exclamation_mark: Last step before we can get things ready for the runtime to compile: we have to tell our runtime what pallets we want it to include (jump to the ``construct_runtime`` macro in ``runtime/src/lib.rs``) Learn more about this ``frame_support`` macro [here](https://substrate.dev/rustdocs/v2.0.0/frame_support/macro.construct_runtime.html):

```bash
construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        /* --snip-- */

        /*** Add This Line ***/
        Contracts: pallet_contracts::{Module, Call, Config, Storage, Event<T>},
    }
);
```

Let's take a second to recap what we've done and make sure everythings working properly:

```bash
SKIP_WASM_BUILD=1 cargo check -p node-template-runtime
```
:heavy_check_mark: We've made a clone of the Substrate node template  
:heavy_check_mark: We've learnt how to navigate our projects directory and how to configure our pallet
:heavy_check_mark: We've included the ```pallet_contracts``` module to our runtime (_a pallet is just Substrate's terminology for a module. A pallet of pallets? Well, a rusty crate ofcourse_ :smirk:)
:heavy_check_mark: We've learnt how to make use of cargo check while making changes to our runtime, to make sure we catch errors while we make progress

**What's next?** 
:mag_right: making use of our new runtime by giving it touchpoints to the outside world :earth_africa:

## Linking things up for Remote Procedure Calls (RPC) :wrench:

To make full use of the Contracts pallet, we need to expose custom endpoints to enable us to read contract state from off-chain. Let's lay down what the next 5 steps will be :
(i) Add ```pallet_contracts_rpc_runtime_api``` to our runtime in ```runtime/Cargo.toml```
(ii) Tell our runtime about the return type of our _getter_ function which will return the current state of execution (from ```ContractsApi```)
(iii) Implement methods from our [ContractsApi trait](https://crates.parity.io/pallet_contracts_rpc_runtime_api/trait.ContractsApi.html) to configure our API endpoints
(iv) Adding an outer node with an RPC API extension. This is the external node that will be used to mimic off-chain interactions setup to receive updates from on-chain changes
(v) Updating the genesis configuration of our chain 

**III. Adding API endpoints:**
(i) :pencil2: Add the following snips to ```runtime/Cargo.toml``` (just like we did in step 3.I):

```bash
[dependencies]
#--snip--
pallet-contracts-rpc-runtime-api = { version = '0.8.0', default-features = false }

[features]
default = ['std']
std = [
    #--snip--
    'pallet-contracts-rpc-runtime-api/std',
]
```

(ii) :eyeglasses: Add this ```use``` statement to ```runtime/src/lib.rs``` :

```bash
use pallet_contracts_rpc_runtime_api::ContractExecResult;
``` 

(iii) :sunglasses: Implement the Contracts API for our runtime by adding some stuff to the ``impl_runtime_apis!``. We'll go through what its doing, but first put the following code snip in your ``runtime/src/lib.rs`` :

```bash
impl_runtime_apis! {
   /* --snip-- */

   /*** Add This Block ***/
    impl pallet_contracts_rpc_runtime_api::ContractsApi<Block, AccountId, Balance, BlockNumber>
        for Runtime
    {
        fn call(
            origin: AccountId,
            dest: AccountId,
            value: Balance,
            gas_limit: u64,
            input_data: Vec<u8>,
        ) -> ContractExecResult {
            let (exec_result, gas_consumed) =
                Contracts::bare_call(origin, dest.into(), value, gas_limit, input_data);
            match exec_result {
                Ok(v) => ContractExecResult::Success {
                    flags: v.flags.bits(),
                    data: v.data,
                    gas_consumed: gas_consumed,
                },
                Err(_) => ContractExecResult::Error,
            }
        }

        fn get_storage(
            address: AccountId,
            key: [u8; 32],
        ) -> pallet_contracts_primitives::GetStorageResult {
            Contracts::get_storage(address, key)
        }

        fn rent_projection(
            address: AccountId,
        ) -> pallet_contracts_primitives::RentProjectionResult<BlockNumber> {
            Contracts::rent_projection(address)
        }
    }
   /*** End Added Block ***/
}
```
As you can see, there's 3 methods we're implementing to our ``impl_runtime_apis!`` macro: ```call```, ```get_storage``` and ```rent_projection``` (see full docs of ContractsApi [here](https://crates.parity.io/pallet_contracts_rpc_runtime_api/trait.ContractsApi.html) to see what other methods you could implement from ContractsApi):

- ```call``` - to perform a call from a specified account to a given contract
- ```get_storage``` - to query a given storage key in a given contract
- ```rent_projection``` - to find out the time a given contract will be able to sustain paying its rent

Before we move on to finish steps (iv) and (v), make sure you've saved everything and checked that it's working:
```bash
SKIP_WASM_BUILD=1 cargo check -p node-template-runtime
```

(iv) :electric_plug: To add the outer node to our exposed runtime API, we'll be going in ```node/Cargo.toml``` to update the node's dependencies. Let's get right to it:

```bash
[dependencies]
jsonrpc-core = '15.0.0'
structopt = '0.3.8'
#--snip--
# *** Add this 2 lines ***
pallet-contracts = '2.0.0'
pallet-contracts-rpc = '0.8.0'
```

:mega: And then, we need to tell our existing RPC about our freshly integrated Contracts pallet and its API that we implemented to make use of it:

```bash
pub fn create_full<C, P>(
    deps: FullDeps<C, P>,
) -> jsonrpc_core::IoHandler<sc_rpc::Metadata> where
    /* --snip-- */
    C: Send + Sync + 'static,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
    /*** Add This Line ***/
    C::Api: pallet_contracts_rpc::ContractsRuntimeApi<Block, AccountId, Balance, BlockNumber>,
    /* --snip-- */
{
    /* --snip-- */

    // Extend this RPC with a custom API by using the following syntax.
    // `YourRpcStruct` should have a reference to a client, which is needed
    // to call into the runtime.
    // `io.extend_with(YourRpcTrait::to_delegate(YourRpcStruct::new(ReferenceToClient, ...)));`

    /*** Add This Block ***/
    io.extend_with(
        ContractsApi::to_delegate(Contracts::new(client.clone()))
    );
    /*** End Added Block ***/
    io
}
```

(v) :hatching_chick: This is the part where we define how we want to initialize our chain's Contracts pallet when it's launched. Most FRAME based pallets have their own a GenesisConfig structs (see [docs](https://substrate.dev/rustdocs/v2.0.0/sc_service/index.html?search=genesisconfig)) and they can be tweaked by going inside the ```testnet_genesis``` function in ```node/src/chain_spec.rs```.

For our purposes, we need to tell our chain that we'll be using [``ContractsConfig``](https://substrate.dev/rustdocs/v2.0.0/pallet_contracts/struct.GenesisConfig.html). This is accomplished by the following line inside ``chain_spec.rs``:
```bash
use node_template_runtime::ContractsConfig;
```

> :bulb: **Note:** our runtime is expecting us to specify this and won't compile until we do. Remember adding ``Contracts: pallet_contracts::{Module, Call, Config, Storage, Event<T>}, `` to our ``runtime/lib.rs`` file in the beggining of this tutorial? That the first thing the rust compiler will check for when it tries to build everything. Looking at the [``Struct pallet_contracts::Config``]https://substrate.dev/rustdocs/v2.0.0/pallet_contracts/struct.Config.html helps grasp the magic behind macros, crates and how modules interact within FRAME.
    
Add the following block inside ``fn testnet_genesis( .. GenesisConfig{..`` (the ``..`` in not actual code, just intended to make things succinct): 

```bash
       /*** Add This Block ***/
        pallet_contracts: Some(ContractsConfig {
            current_schedule: pallet_contracts::Schedule {
                    enable_println,
                    ..Default::default()
            },
        }),
```


## Compile and run node :checkered_flag:
Let's check if it all works! :sparkles: Use this command to compile your node first:

```bash
WASM_BUILD_TOOLCHAIN=nightly-2020-10-05 cargo build --release
```
And (assuming it compiled without errors) this one to run it:
```bash
./target/release/node-template --dev --tmp
``` 

If everything worked, your chain is successfully producing blocks, you should be able to go to https://polkadot.js.org/apps/#/contracts and upload a contract to your chain. :smiley: :clap:

[./contract-ui.png]

# Acknowledgements :pray:

Dan Forbes from Parity Technologies for providing the [basis of this tutorial](https://substrate.dev/docs/en/tutorials/add-contracts-pallet/). 


:bulb: Ideas for next tutorials:
- Interacting with Contracts using WASM
- Fundemental concepts (runtimes, pallets, macros, weights, governance and GenesisConfig)
- Design decisions (why implement certain pallets, how to customize logic, use-case specific architecture)
