# Rust installieren

Auf [rustup.rs](http://rustup.rs) gibt es einen Installer für Windows bzw.
ein Script für Unix zum Installieren der Toolchain und des Packagemanagers 
(Cargo)

Anschließend kann über die Kommandozeile (ggf muss selbige einmal aktualisiert werden) 
```
> rustup show 

< Default host: x86_64-pc-windows-msvc
< rustup home:  C:\Users\schle\.rustup
< installed toolchains
< --------------------
< 
< stable-x86_64-pc-windows-msvc (default)
< nightly-x86_64-pc-windows-msvc
< 
< active toolchain
< ----------------
< 
< stable-x86_64-pc-windows-msvc (default)
< rustc 1.65.0 (897e37553 2022-11-02)
```

bzw. 
```
> cargo --version

< cargo 1.65.0 (4bc8f24d3 2022-10-20)
```
überprüft werden ob die Installation erfolgreich war bzw welche toolchain installiert ist

## Managen einer (anderen) Toolchain

Bei manchen Projekten kann es interessant (oder nötig) sein weitere Toolchains
zu installieren.

Ganz prominent ist hierbei die "nightly" Toolchain, welche anders als "stable" häufig experimentelle (aber wichtige/praktische)
Features enthält.

```
rustup target list
```
Liefert alle existierenden Toolchains

```
rustup toolchain install nightly-msvc
```
Installiert z.B. die "Nightly" Toolchain für den Microsoft-Compiler. Das Target-Triple wird inferiert.
Äquivalent wäre also
```
rustup toolchain install nightly-x86_64-pc-windows-msvc
```
```
rustup toolchain default nightly-msvc
```
Setzt eine Toolchain als default und (sofern noch nicht existent) installiert diese.
