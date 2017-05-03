build:
	cargo build
	cp target/debug/magnusi-tech-server ./

run:
	cargo run

clean:
	cargo clean
	rm -f ./magnusi-tech-server
