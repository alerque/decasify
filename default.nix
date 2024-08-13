# SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
# SPDX-License-Identifier: LGPL-3.0-only
{ lib
, naersk
, stdenv
, hostPlatform
, targetPlatform
, cargo
, rustc
}:
let
  cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
in
naersk.lib."${targetPlatform.system}".buildPackage rec {
  src = ./.;
  buildInputs = [
    cargo
    rustc
  ];
  cargoBuildOptions = final: final ++ [ "--features cli" ];
  singleStep = true;
  copyLibs = true;
  name = cargoToml.package.name;
  version = cargoToml.package.version;
  meta = with lib; {
    description = cargoToml.package.description;
    homepage = cargoToml.package.homepage;
    license = with licenses; [ lgpl3 ];
    maintainers = with maintainers; [ ];
  };
}
