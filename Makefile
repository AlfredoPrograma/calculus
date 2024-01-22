run:
	cargo run

test:
	cargo test

coverage:
	cargo tarpaulin --out Html && xdg-open tarpaulin-report.html