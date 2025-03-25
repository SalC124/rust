{
  description = "rust dev shell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-24.11";
  };

  outputs =
    { nixpkgs, ... }:
    let
      pkgs = nixpkgs.legacyPackages."x86_64-linux";
    in
    {
      devShells."x86_64-linux".default = pkgs.mkShell {

        packages = with pkgs; [
          rustc
          cargo
          zellij
        ];


        shellHook = ''
          alias rzjk="zellij d --force 'coding_rust_(btw)_in_nvim_(btw)_in_nixos_(btw)'"
          alias bye="zellij d --force 'coding_rust_(btw)_in_nvim_(btw)_in_nixos_(btw)' && exit"

          echo "./pane-start.sh
./rust_layout.kdl" > .gitignore

          echo "zellij action rename-session 'coding_rust_(btw)_in_nvim_(btw)_in_nixos_(btw)' && zsh" > pane-start.sh
          echo 'layout {
    pane {
        cwd "./projects/"
        command "~/Code/Personal/rust/pane-start.sh"
        size "75%"
    }
    pane {
        cwd "./projects/"
        command "zsh"
    }
    pane size=1 borderless=true {
        plugin location="zellij:status-bar"
    }
}
' > rust_layout.kdl

          zellij d --force 'coding_rust_(btw)_in_nvim_(btw)_in_nixos_(btw)' || echo ""
          chmod +x ./pane-start.sh
          zellij --layout ./rust_layout.kdl
        '';
      };
    };
}
