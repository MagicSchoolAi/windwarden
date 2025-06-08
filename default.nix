{ pkgs ? import
    (fetchTarball {
      name = "jpetrucciani-2025-06-07";
      url = "https://github.com/jpetrucciani/nix/archive/2211ccdc90d9009b93198888a675cb8b97eeadb0.tar.gz";
      sha256 = "0sg7sc2a13wgjf3wwshypjpmm61vs2gczn9kcc21n0gqqbpd4g0c";
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
  NIXUP = "0.0.9";
})) // { inherit scripts; }
