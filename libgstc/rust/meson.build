rust_enabled = add_languages('rust', required : get_option('enable-rust'))

if rust_enabled
  gstc_rust_sources = [
    'gstc/src/lib.rs',
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

  pkgconfig.generate(
    gstc_rust_lib,
    description : 'GStreamer Rust Client library to control Gstd'
  )
else
  message('Rust compiler not found; skipping libgstc Rust API')
endif
