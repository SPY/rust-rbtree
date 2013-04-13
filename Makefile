BIN=rbtree-test

clean: 
	rm $(BIN)

build:
	rustc rbtree.rs --test -o $(BIN)

test: build
	./$(BIN)
