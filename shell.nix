with import <nixpkgs> {};

pkgs.mkShell rec {
    buildInputs = with pkgs; [ nodejs-14_x yarn electron ];

    ELECTRON_SKIP_BINARY_DOWNLOAD = 1;

    shellHook = ''
      NODE_MODULES=(.yarn/unplugged/electron*)

      for MODULE in $NODE_MODULES; do
        FILE="$MODULE/node_modules/electron/dist/electron"

        if [ -f "$FILE" ]; then
          rm "$FILE"
        fi

        ln -s ${pkgs.electron}/bin/electron "$FILE"
      done
    '';
}
