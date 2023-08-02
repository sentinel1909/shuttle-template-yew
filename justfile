# just task runner configuration for shuttle-template-yew

serve:
  cd frontend && trunk serve --open

build-release:
  cd frontend && trunk build --release
