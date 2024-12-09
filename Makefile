Z_FLAGS=-Z build-std=core,compiler_builtins \
		-Z build-std-features=compiler-builtins-mem

TARGET=-x86_64-HLeos.json

.PHONY : all

all : $(TARGET)
	cargo build $(Z_FLAGS) --target $(TARGET)