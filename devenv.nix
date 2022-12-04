{ inputs, pkgs, ... }:

let
  pkgs-stable = import inputs.nixpkgs-stable { inherit (pkgs.system); };
in
{
  packages = [
    pkgs-stable.git
    pkgs-stable.gh
  ];
}
