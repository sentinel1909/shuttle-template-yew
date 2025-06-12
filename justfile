# just task runner configuration for shuttle-template-yew

# use Powershell instead of sh
set shell := ["powershell.exe", "-c"]

dev:
  cd frontend; trunk serve --open

build:
  cd frontend; trunk clean; trunk build --release

local:
  shuttle run

deploy:
  shuttle deploy
