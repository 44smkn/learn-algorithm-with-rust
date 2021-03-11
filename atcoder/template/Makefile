.SILENT:
run: 
	rustc -C debuginfo=0 -C opt-level=3 -o DIRECTORY main.rs
	time -p ./DIRECTORY
	rm -f ./DIRECTORY