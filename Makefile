test: test_serial test_full_tree 

test_serial:
	cargo test -- -q --skip full_tree

test_full_tree:
	cargo test -- -q full_tree