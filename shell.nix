{
  pkgs ? import <nixpkgs> { },
}:

let
  nativeLibs = with pkgs; [
    glib
    gtk3
    webkitgtk_4_1
    libsoup_3
    openssl
    xdotool
    libayatana-appindicator
  ];
in
pkgs.mkShell {
  packages =
    with pkgs;
    [
      cargo
      rustc
      clippy
      rustfmt
      rust-analyzer
      dioxus-cli
      pkg-config
      gsettings-desktop-schemas
    ]
    ++ nativeLibs;

  RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";

  shellHook = ''
    export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath nativeLibs}''${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
    # GTK/GLib schemas so file dialogs (rfd) don't abort at runtime
    export XDG_DATA_DIRS="${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}''${XDG_DATA_DIRS:+:$XDG_DATA_DIRS}"
    export WEBKIT_DISABLE_DMABUF_RENDERER=1
  '';
}
