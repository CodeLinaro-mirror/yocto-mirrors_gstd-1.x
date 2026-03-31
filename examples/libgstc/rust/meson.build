if is_variable('lib_gstc_rust_dep')
  rustc = find_program('rustc')
  rust_target_libdir = run_command(rustc, '--print', 'target-libdir', check : true).stdout().strip()

  rust_examples = [
    ['simple_pipeline', 'simple_pipeline_rust.rs'],
    ['dynamic_property_change', 'dynamic_property_change_rust.rs'],
    ['gapless_playback', 'gapless_playback_rust.rs'],
    ['mp4_recording', 'mp4_recording_rust.rs'],
    ['pipeline_lifecycle', 'pipeline_lifecycle_rust.rs'],
    ['wait_on_bus', 'wait_on_bus_rust.rs'],
  ]

  foreach ex : rust_examples
    executable(ex[0], ex[1],
      rust_args : ['--edition=2021'],
      dependencies : [lib_gstc_rust_dep],
      build_rpath : rust_target_libdir,
      install_rpath : rust_target_libdir,
      install : false,
    )
  endforeach
else
  message('Rust libgstc dependency not found; skipping Rust examples')
endif
