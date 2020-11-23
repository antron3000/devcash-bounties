Bounty adress: _will be updated soon_ 

### Here's the original bounty (by Antoine) 

**Task**: Write a useful tutorial

**Description**: Teach something valuable 

**Devcash reward**: 100, 000 {D}

### My Tutorial
:warning: _work in progress_

A short beginners guide for building with [Substrate](https://substrate.dev/).

The aim of this tutorial is to: (1) provide a brief overview of Subtrate's building blocks and key notions and (2) provide a step by step for adding custom functionality to a template node.

It will cover: 
- Basic setup and tools 
- A step-by-step to launch a Substrate chain with a collection of pallets that provides functionality for the runtime to deploy and execute WebAssembly smart-contracts.

# 1. Getting started
## Installing Substrate node templates
Make sure you're environment is all setup for using Rust. Refer to [this tutorial] (https://substrate.dev/docs/en/knowledgebase/getting-started/) if it isn't already.

In this first step, we're going to clone the node template and compile it. This can take a little while depending on your hardware, don't panic (took me 25m57s on a fairly powerful laptop).

1. Clone the Node Template
```bash 
git clone -b v2.0.0 --depth 1 https://github.com/substrate-developer-hub/substrate-node-template
```

2. Initialize WASM
```bash
make init
```

3. Compile it
```bash 
make build
```

While it compiles, let's understand what this template is and what it will do for us.

The template is basically boiler plate from which you can customize your runtime. Referred to as a "FRAME-based Substrate node", "FRAME" is short for: _a Framework for Runtime Aggregation of Modularized Entities_. Its essentially a pretty mightly library of libraries for building things with Substrate. 

In what we're compiling, our runtime has the following pallets configured (see [docs](https://github.com/substrate-developer-hub/substrate-node-template/blob/cbf28f104237648512e8e3b6af2126e840d56243/runtime/src/lib.rs#L269-L287)). We can find out what each one does by looking at their documentation:

- [``frame_system``](https://crates.parity.io/frame_system/index.html) - provides low-level access to core types and cross-cutting utilities.
- [``pallet_randomness_collective_flip``](https://crates.parity.io/pallet_randomness_collective_flip/index.html) - provides a ``random`` function that generates low-influence random values based on the block hashes from the previous 81 blocks.
- [``pallet_timestamp``](https://crates.parity.io/pallet_timestamp/index.html) - provides functionality to get and set the on-chain time.
- [``pallet_aura``](https://crates.parity.io/pallet_aura/index.html) - extends Aura consensus by managing offline reporting.
- [``pallet_grandpa``](https://crates.parity.io/pallet_grandpa/index.html) - GRANDPA Consensus module for runtime. This manages the GRANDPA authority set ready for the native code. 
- [``pallet_balances``](https://crates.parity.io/pallet_balances/index.html) - provides functionality for handling accounts and balances.
- [``pallet_transaction_payment``](https://crates.parity.io/pallet_transaction_payment/index.html) - provides the basic logic needed to pay the absolute minimum amount needed for a transaction to be included. 
- [``pallet_sudo``](https://crates.parity.io/pallet_sudo/index.html) - allows for a single account (called the "sudo key") to execute dispatchable functions that require a Root call or designate a new account to replace them as the sudo key. 
- [``pallet_template``](https://crates.parity.io/pallet_template/index.html) - _doesn't do anything_

Learn more by checking out the Substrate Developer Hub [docs](https://substrate.dev/docs/en/).

## Explore the template setup
Below is the node templates directory tree. Get familiar with the structure so you know where to find the files we'll be modifying in this tutorial.

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
## Running your node



# 2. Launching your node

Once everything has compiled, it's time to test our node. Running this command will launch it in development mode:
``./target/release/node-template --dev --tmp``

If all is working fine, you should see blocks being created in your terminal.

To see what's happening with a better GUI, you can head to: https://polkadot.js.org/apps/ and switch to Local Node. 

# 3. Importing a new pallet
We'll be importing a number of dependencies to implement the [``pallets_contracts``](https://substrate.dev/rustdocs/v2.0.0/pallet_contracts/index.html) pallet. These will be:

```bash
pallet-contracts = { version = '2.0.0', default_features = false }
pallet-contracts-primitives = { version = '2.0.0', default_features = false }
pallet-contracts-rpc-runtime-api = { version = '0.8.0', default-features = false }
```

## Setting things up for our runtime

**1. Update ``runtime/Cargo.toml`` by adding the following:**
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

## Setting up the pallet
**2. Configure the pallet:**

For the Contracts Pallet, we can find out how to configure it by looking at its ``Trait`` types (see the [docs](https://substrate.dev/rustdocs/v2.0.0/pallet_contracts/trait.Trait.html)). 

In ``runtime/src/lib.rs``, paste the following configurations for the Nick's Pallet:

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

Last step before we can get things ready for the runtime to compile: we have to tell our runtime what pallets it needs to include (jump to the ``construct_runtime`` macro in ``runtime/src/lib.rs``) Learn more about this ``frame_support`` macro [here](https://substrate.dev/rustdocs/v2.0.0/frame_support/macro.construct_runtime.html):

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
## Compile and run node
Let's check if it all works! Use this command to compile your node:

```bash
WASM_BUILD_TOOLCHAIN=nightly-2020-10-05 cargo build --release
```
# Exposing the Contracts API
(TODO)

To run your node, assuming nothing went wrong above use this command:
```bash
./target/release/node-template --dev --tmp
```

Now head to Polkadot's GUI and you should be able to  submit a ``nicks`` extrinsic.





:bulb: Ideas for next tutorials:
- Fundemental concepts (runtimes, pallets, macros, weights, governance and GenesisConfig)
- Design decisions (why implement certain pallets, how to customize logic, use-case specific architecture)
