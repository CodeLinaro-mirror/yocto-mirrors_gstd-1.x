rust_enabled = add_languages('rust', required : false)

if rust_enabled
  subdir('gstc')
else
  message('Rust compiler not found; skipping libgstc Rust API')
endif
