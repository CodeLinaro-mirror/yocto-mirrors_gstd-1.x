gstc_rust_sources = [
  'src/lib.rs',
]

gstc_rust_lib = static_library('gstc_rust',
  gstc_rust_sources,
  rust_crate_type : 'rlib',
  rust_args : ['--edition=2021'],
  install : true,
  install_dir : lib_install_dir,
)

lib_gstc_rust_dep = declare_dependency(
  link_with : gstc_rust_lib,
)

# Generate pkgconfig file
pkgconfig.generate(gstc_rust_lib, description : 'GStreamer Rust Client library to control Gstd')
