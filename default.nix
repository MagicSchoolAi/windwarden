{ pkgs ? import
    (fetchTarball {
      name = "MagicSchoolAi-2025-09-01";
      url = "https://github.com/MagicSchoolAi/nix-ops/archive/b6fd14a095311ca241f71ae0331e8f495e6964e5.tar.gz";
      sha256 = "0jxd668fa85gbygxjslp8ak5qg5dxjfzxl0333vynzdvamb6447z";
    })
    { }
}:
let
  name = "tailwarden";

  tools = with pkgs; {
    cli = [
      jfmt
      nixup
      claude-code
    ];
    rust = [
      clippy
      cargo
      clang
      rust-analyzer
      rustc
      rustfmt
      # deps
      pkg-config
      openssl
    ];
    scripts = pkgs.lib.attrsets.attrValues scripts;
  };

  scripts = with pkgs; { };
  paths = pkgs.lib.flatten [ (builtins.attrValues tools) ];
  env = pkgs.buildEnv {
    inherit name paths; buildInputs = paths;
  };
in
(env.overrideAttrs (_: {
  inherit name;
  NIXUP = "0.0.10";
})) // { inherit scripts; }
