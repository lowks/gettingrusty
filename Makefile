.PHONY: clean all
all:
	rm -rf bin
	mkdir bin
	cargo run
clean:
	rm -rf bin 
