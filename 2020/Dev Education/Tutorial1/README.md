Bounty adress: _will be updated soon_ 

### Here's the original bounty (by Antoine) 

**Task**: Write a useful tutorial

**Description**: Teach something valuable 

**Devcash reward**: 100, 000 {D}

### My Tutorial
:warning: _work in progress_

A guide through building using [Substrate](https://substrate.dev/) 

The aim of this tutorial is to provide an overview of things to consider when diving into building with Subtrate.

It will cover: 
- Basic setup and tools 
- Design decisions and fundemental concepts (runtimes, pallets, macros, weights, governance and genesisconfig)
- An step-by-step to build a a chain whose genesis will be a multisig account and can schedule runtime upgrades

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

## Setting up runtime
We'll be creating a pallet that can name accounts and keep track of them, called the [Nicks Pallet](https://substrate.dev/rustdocs/v2.0.0/pallet_nicks/index.html).  

1. Update ``runtime/Cargo.toml`` by adding the following
```bash
pallet-nicks = { default-features = false, version = '2.0.0' }
```
and: 

```bash
[features]
default = ["std"]
std = [
    #--snip--
    'pallet-nicks/std',
    #--snip--
]
```
You can run the following command to check whether the pallet has  been imported with no errors: 
```bash
cargo check -p node-template-runtime
```

## Setting up the pallet
2. Configure the pallet:
For the Nicks Pallet, we can find out how to configure it by looking at is ``Trait`` (see the [docs](https://substrate.dev/rustdocs/v2.0.0/pallet_nicks/trait.Trait.html)). Here are the different trait types:

- ``Currency`` - the currency type for name deposits
- ``ReservationFee`` - amount required to reserve
- `` Slashed`` - callback invoked when a deposit is forfeited
- ``ForceOrigin`` - used to idenfity the pallet's admin
- ``MinLength`` - the mininum length for a name
- ``MaxLength``  - the maximum length for a name

In ``runtime/src/lib.rs``, paste the following configurations for the Nick's Pallet:

```bash
parameter_types! {
    // Choose a fee that incentivizes desireable behavior.
    pub const NickReservationFee: u128 = 100;
    pub const MinNickLength: usize = 8;
    // Maximum bounds on storage are important to secure your chain.
    pub const MaxNickLength: usize = 32;
}

impl pallet_nicks::Trait for Runtime {
    // The Balances pallet implements the ReservableCurrency trait.
    // https://substrate.dev/rustdocs/v2.0.0/pallet_balances/index.html#implementations-2
    type Currency = pallet_balances::Module<Runtime>;

    // Use the NickReservationFee from the parameter_types block.
    type ReservationFee = NickReservationFee;

    // No action is taken when deposits are forfeited.
    type Slashed = ();

    // Configure the FRAME System Root origin as the Nick pallet admin.
    // https://substrate.dev/rustdocs/v2.0.0/frame_system/enum.RawOrigin.html#variant.Root
    type ForceOrigin = frame_system::EnsureRoot<AccountId>;

    // Use the MinNickLength from the parameter_types block.
    type MinLength = MinNickLength;

    // Use the MaxNickLength from the parameter_types block.
    type MaxLength = MaxNickLength;

    // The ubiquitous event type.
    type Event = Event;
}
```

Last step before we can get the Nicks Pallet ready for the runtime to compile. We have to tell our runtime it needs to include the Nicks Pallet (jump to the ``construct_runtime`` macro in ``runtime/src/lib.rs``):

```bash
construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        /* --snip-- */

        /*** Add This Line ***/
        Nicks: pallet_nicks::{Module, Call, Storage, Event<T>},
    }
);
```
## Compile and run node
Let's check if it all works! Use this command to compile your node:

```bash
WASM_BUILD_TOOLCHAIN=nightly-2020-10-05 cargo build --release
```
To run your node, assuming nothing went wrong above use this command:
```bash
./target/release/node-template --dev --tmp
```

Now head to Polkadot's GUI and you should be able to  submit a ``nicks`` extrinsic.







