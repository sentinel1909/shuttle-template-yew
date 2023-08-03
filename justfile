# just task runner configuration for shuttle-template-yew

dev:
  cd frontend && trunk serve --open

build-release:
  cd frontend && trunk clean && trunk build --release

shuttle-run:
  cargo shuttle run

shuttle-deploy:
  rm -r dist && cp -r server/dist/ dist && cargo shuttle deploy
