let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs {};
in
pkgs.mkShell {

  buildInputs = with pkgs; [
    cargo-watch
    libiconv
    rustup
  ];

  LANG = "en_US.UTF8";

  shellHook = '''';
}
