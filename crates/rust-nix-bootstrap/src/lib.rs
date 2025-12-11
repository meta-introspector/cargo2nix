pub trait NixPathsProvider {
    fn glibc_dev(&self) -> &'static str;
    fn gcc_path(&self) -> &'static str;
    fn gcc_cpp_include(&self) -> &'static str;
    fn openssl_include(&self) -> &'static str;
    fn openssl_lib(&self) -> &'static str;
    fn zlib_include(&self) -> &'static str;
    fn bzip2_include(&self) -> &'static str;
    fn lz4_include(&self) -> &'static str;
    fn lz4_lib(&self) -> &'static str;
    fn zstd_include(&self) -> &'static str;
    fn zstd_lib(&self) -> &'static str;
    fn llvm_config(&self) -> &'static str;
    fn libclang_path(&self) -> &'static str;
    fn llvm_config_path(&self) -> &'static str;
}

pub struct HardcodedNixPaths;

impl NixPathsProvider for HardcodedNixPaths {
    fn glibc_dev(&self) -> &'static str {
        "/nix/store/gi4cz4ir3zlwhf1azqfgxqdnczfrwsr7-glibc-2.40-66-dev"
    }

    fn gcc_path(&self) -> &'static str {
        "/nix/store/82kmz7r96navanrc2fgckh2bamiqrgsw-gcc-14.3.0"
    }

    fn gcc_cpp_include(&self) -> &'static str {
        "/nix/store/82kmz7r96navanrc2fgckh2bamiqrgsw-gcc-14.3.0/include/c++/14.3.0"
    }

    fn openssl_include(&self) -> &'static str {
        "/nix/store/ydrckgnllgg8nmhdwni81h7xhcpnrlhd-openssl-3.6.0-dev/include"
    }

    fn openssl_lib(&self) -> &'static str {
        "/nix/store/ydrckgnllgg8nmhdwni81h7xhcpnrlhd-openssl-3.6.0-dev/lib"
    }

    fn zlib_include(&self) -> &'static str {
        "/nix/store/hqvsiah013yzb17b13fn18fpqk7m13cg-zlib-1.3.1-dev/include"
    }

    fn bzip2_include(&self) -> &'static str {
        "/nix/store/q1a3bjhg3b4plgb7fk7zis1gi09rbi1d-bzip2-1.0.8-dev/include"
    }

    fn lz4_include(&self) -> &'static str {
        "/nix/store/n9gqsgvq7vjzbll7mps9pqkmy1hj1gcq-lz4-1.9.4-dev/include"
    }

    fn lz4_lib(&self) -> &'static str {
        "/nix/store/9awv9f5xrvfb85jxk4wlh9n138hpnlpx-lz4-1.10.0-lib/lib"
    }

    fn zstd_include(&self) -> &'static str {
        "/nix/store/cgcbi8wsxhcf8kkzn78h6h158adpfzbc-zstd-1.5.5-dev/include"
    }

    fn zstd_lib(&self) -> &'static str {
        "/nix/store/ry1jx5972j5clvqapx33v9imba8ywvq6-zstd-1.5.5/lib"
    }

    fn llvm_config(&self) -> &'static str {
        "/nix/store/v9cr3iv7wnrkjy1s3z1fi7wpkl7sy4hx-llvm-21.1.2-dev/bin/llvm-config"
    }

    fn libclang_path(&self) -> &'static str {
        "/nix/store/10mkp77lmqz8x2awd8hzv6pf7f7rkf6d-clang-19.1.7-lib/lib/"
    }

    fn llvm_config_path(&self) -> &'static str {
        "/nix/store/b5bmnvk17mq8qm5b8bpi9fkyr5g2d2m4-llvm-21.1.2/lib"
    }
}