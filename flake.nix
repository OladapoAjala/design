{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    systems.url = "github:nix-systems/default";
    devenv.url = "github:cachix/devenv";
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = { self, nixpkgs, devenv, systems, ... } @ inputs:
    let
      forEachSystem = nixpkgs.lib.genAttrs (import systems);
    in
    {
      packages = forEachSystem (system: {
        devenv-up = self.devShells.${system}.default.config.procfileScript;
      });

      devShells = forEachSystem
        (system:
          let
            pkgs = nixpkgs.legacyPackages.${system};
          in
          {
            languages = {
              go = {
                enable = true;
                version = "1.21.5";
              };
            };


            default = devenv.lib.mkShell {
              inherit inputs pkgs;
              modules = [
                {
                  packages = with pkgs; [
                    # Protobuf
                    protobuf
                    protoc-gen-go
                    protoc-gen-go-grpc

                    # GRPC
                    grpcurl
                  ];

                  scripts = {
                    thanos-grpc.exec = ''
                      cd thanos/proto/thanos &&
                      protoc --go_out=. --go_opt=paths=source_relative \                                                                                                                                                           INT ✘  ▼  impure  
                        --go-grpc_out=. --go-grpc_opt=paths=source_relative \
                        thanos/thanos.proto
                    '';
                  };

                  processes = {
                    minikube.exec = "minikube start --cpus=max";
                  };
                }
              ];
            };
          });
    };
}
