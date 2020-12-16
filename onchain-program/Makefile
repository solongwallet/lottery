TARGET := solong-lottery
SRC_FILES := $(shell find ./src -type  f | grep "\.rs"  )
#SRC_FILES += main.go flag.go config_xml.go

#VERSION := $(shell git log  | head -n 1 | awk '{print $2}')

.DEFAULT:all

all : $(TARGET)
	@echo "[SUCCESS] Done build bpf"


$(TARGET): $(SRC_FILES)
	clear	
	@echo "src :"$(SRC_FILES)
	cargo build-bpf

test: $(SRC_FILES)
	clear
	cargo test-bpf
