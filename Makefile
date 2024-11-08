RUST_OUT=target/x86_64-unknown-none/debug

.PHONY: all clean

all:
	RUSTFLAGS="-C relocation-model=static" cargo build
	git clone https://github.com/limine-bootloader/limine.git \
		--branch=v8.x-binary --depth=1
	make -C limine
	mkdir -p iso
	mkdir -p iso/EFI/BOOT
	mkdir -p iso/boot/limine
	cp -v $(RUST_OUT)/k64 iso/boot
	cp -v limine.conf iso/boot/limine
	cp -v limine/limine-bios.sys \
		limine/limine-bios-cd.bin \
		limine/limine-uefi-cd.bin \
		iso/boot/limine
	xorriso -as mkisofs -b boot/limine/limine-bios-cd.bin 			\
		-no-emul-boot -boot-load-size 4 -boot-info-table 					\
		--efi-boot boot/limine/limine-uefi-cd.bin               	\
		-efi-boot-part --efi-boot-image --protective-msdos-label	\
		iso -o $(RUST_OUT)/k64.iso
	limine/limine bios-install $(RUST_OUT)/k64.iso

test: all
	mkdir log
	../../toolchains/bochs-x86_64/bochs-2.8/bochs -qf bochsrc.txt

clean:
	rm -rf limine
	rm -rf iso
	rm -rf log
	cargo clean
