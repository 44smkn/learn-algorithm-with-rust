define STACK_INPUT
1 2 + 3 4 - *
上記を入力をしてください
endef

export STACK_INPUT
stack:
	rustc stack.rs
	echo "$${STACK_INPUT}"
	./stack