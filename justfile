# just task runner configuration for shuttle-template-yew

run-dev:
  cd frontend && trunk serve --open

build-release:
  cd frontend && trunk clean && trunk build --release

project-init:
  cargo shuttle init

shuttle-run:
  cargo shuttle run

shuttle-deploy:
  cargo shuttle project restart && cargo shuttle deploy
