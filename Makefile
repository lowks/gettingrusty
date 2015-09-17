.PHONY: clean all
all:
	mkdir bin
	cargo run
clean:
	rm -rf bin 
