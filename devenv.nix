# see https://devenv.sh for documentation

{ pkgs, lib, config, inputs, ... }:
{
  packages = [ pkgs.git pkgs.cargo-nextest ];
  languages.rust.enable = true;

  enterTest = ''
      cargo nextest run
    '';
}
