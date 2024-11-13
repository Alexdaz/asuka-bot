run:
	cargo run

clean:
	cargo clean

mycpu:
	cargo rustc --release -- -C target-cpu=znver3

#Change native by the CPU server.
prod:
	cargo rustc --release -- -C target-cpu=native


#############
# PROFILING #
#############

memprofile = RUSTFLAGS='-g' cargo build --release --bin $(1); \
                heaptrack -o /tmp/heaptrack.pbqff.%p.zst target/release/$(1)

memprofile.asuka:
	$(call memprofile,asuka)