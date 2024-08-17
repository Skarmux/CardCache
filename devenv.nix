{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/basics/
  # env.GREET = "devenv";

  # https://devenv.sh/packages/
  packages = with pkgs; [
    git
    pre-commit
  ];

  # https://devenv.sh/languages/
  # languages.rust.enable = true;

  # https://devenv.sh/processes/
  processes = {
    # backend
    backend.exec = "cargo-watch -x run --workdir backend";

    # frontend
    tailwind.exec = "tailwindcss -c frontend/tailwind.config.js -i frontend/tailwind.in.css -o frontend/tailwind.css --minify --watch";
    frontend.exec = "trunk serve frontend/index.html";
  };

  # https://devenv.sh/services/
  # services.postgres.enable = true;

  # https://devenv.sh/scripts/
  # scripts.hello.exec = ''
  #   echo hello from $GREET
  # '';

  # enterShell = ''
  #   hello
  #   git --version
  # '';

  # https://devenv.sh/tests/
  # enterTest = ''
  #   echo "Running tests"
  #   git --version | grep --color=auto "${pkgs.git.version}"
  # '';

  # https://devenv.sh/pre-commit-hooks/
  pre-commit.hooks.shellcheck.enable = false;

  # See full reference at https://devenv.sh/reference/options/
}
