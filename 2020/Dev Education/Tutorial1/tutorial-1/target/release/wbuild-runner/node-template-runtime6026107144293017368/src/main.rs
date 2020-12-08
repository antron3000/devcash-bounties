
				//! This is automatically generated code by `substrate-wasm-builder`.

				use substrate_wasm_builder::build_project_with_default_rustflags;

				fn main() {
					build_project_with_default_rustflags(
						"/home/alansky/Dev/Parity/substrate-node-template/target/release/build/node-template-runtime-99dc17bb0ac2e6d9/out/wasm_binary.rs",
						"/home/alansky/Dev/Parity/substrate-node-template/runtime/Cargo.toml",
						"-Clink-arg=--export=__heap_base -C link-arg=--import-memory ",
					)
				}
			