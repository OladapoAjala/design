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

              rust = {
                enable = true;
                channel = "stable";
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

                    # Kubernetes
                    skaffold
                    kpt
                    kubernetes-helm
                  ];

                  scripts = {
                    thanos-grpc.exec = ''
                      cd thanos/proto/thanos &&
                      protoc --go_out=. --go_opt=paths=source_relative --go-grpc_out=. --go-grpc_opt=paths=source_relative thanos.proto
                    '';
                  };

                  process.before = ''
                    if ! minikube status > /dev/null 2>&1; then
                      echo 'Minikube is not running. Starting Minikube...'
                      minikube start --cpus=max
                    else
                      echo 'Minikube is already running.'
                    fi

                    if ! kubectl get namespace design > /dev/null 2>&1; then
                      echo "Namespace design does not exist. Creating it..."
                      kubectl create namespace design 
                    else
                      echo "Namespace design already exists."
                    fi
                  '';

                  processes = {
                    skaffold.exec = "cd rate-limiter; skaffold  --force-colors dev";
                    tunnel.exec = "minikube tunnel";
                  };
                }
              ];
            };
          });
    };
}
