sed -i 's/let _ = fs::remove_file(&vm_write.akashic.file_path);/let _ = fs::remove_file(\&vm_write.akashic.file_path);/' experiments/chimera-lang/tests/akashic_test.rs
sed -i 's/let _ = fs::remove_file(&vm.akashic.file_path);/let _ = fs::remove_file(\&vm.akashic.file_path);/' experiments/chimera-lang/tests/akashic_test.rs
