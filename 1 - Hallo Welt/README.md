# Projekt initialisieren
### Wichtig am Rande:
Rust bevorzugt **lower_snake_case**! Wird das nicht eingehalten bedankt sich der Compiler mit Warnungen 
und kann im schlimmsten Fall, wenn er Streng eingestellt ist, den Code nicht übersetzten.
Dieses Verhalten kann aber deaktiviert werden ;) 


### Erstellen eines neuen Projektes
Wir bewegen uns in den Ordner, in dem wir das Projekt anlegen wollen und führen folgenden Befehl aus

``` 
cargo new name_generator 
```
Anschließend erstellt der Package-Manager "Cargo" alle nötigen Dateien um ein Ausführbares Program (in unserem Fall
den "name_generator") zu erstellen
> Der Zusatz `--lib` erstellt nur die beötigten Dateien um eine Bibliothek anzulegen, welche in anderen Projekten verwendet werden kann.

Navigieren wir nun in den neu erstellten Ordner unseres Projektes und führen
```
> cd name_generator
> cargo run
```
Der Compiler sollte folgende Zeilen in der Kommandozeile ausgegeben haben
```
<   Compiling name_generator v0.1.0 (C:\Users\schle\IdeaProjects\Rust Workshop\1 - Hallo Welt\name_generator)
<     Finished dev [unoptimized + debuginfo] target(s) in 0.33s
<      Running `target\debug\name_generator.exe`
< Hello, world!
```
Die ersten 3 Zeilen sind Informationen des Compilers und an dieser Stelle noch nicht interesant.
Die 4. Zeile "Hello, world!" ist hingegen die Ausgabe unseres ersten Rust-Programms!

### Anpassen des Sourcecodes
Da wir dort nicht `Hello, world!` stehen haben wollen, sondern etwas sinnvolles, schauen wir uns hier 
die, von Cargo erstellten Dateien etwas genauer an:

```shell
name_generator/             # Projektordner
    src/                    # Haupt-Ordner für Quellcode
        main.rs             # Haupt Programm Einstieg (zu den verschiedenen "festen" Dateinamen später mehr)
    target/                 # Kompilat/Output-Ordner nach cargo build / cargo run
    Cargo.lock              # NICHT EDITIEREN: Hier "merkt" sich Cargo den Zustand des Projektes
    Cargo.toml              # Projektkonfiguration, ähnlich zu package.json bei nodeJS oder requirements.txt
```
Für den Anfang reicht es uns die Dateien `src/main.rs` und `Cargo.toml` zu betrachten.

#### Cargo.toml
```Toml
[package]
name = "name_generator"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```
In dieser "Projektdatei" können verschiedenste Einstellungen vorgenommen werden. Soll zum Beispiel das Projekt auf Bibliotheken zugreifen
können diese unter `dependencies` angefügt werden.

Früher oder später möchte man sein Programm auf eine intelligente Art und Weise mit verschiedenen Parametern
starten können, weshalb wir unser Projekt nun um die Bibliothek `clap` (- Command Line Argument Parser) erweitern:
```Toml
[package]
name = "name_generator"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
clap = { version = "*", features = ["derive","cargo"] }
```
Dabei geben wir mit dem Asterisk `*` an, dass uns die Version von Clap egal ist und immer die neueste verwendet werden soll.
`^3.2` würde bedeuten, dass MINDESTENS die Version 3.2 verwendet werden soll, davon allerdings die neueste (3.2.23).
`features` sind Optionen die an einzubindende Bibliotheken übermittelt werden. In unserem Fall wollen wir
manche Fähigkeiten von Clap "ableiten"... dies sind üblicherweise Macros die eingebunden werden.


Alternativ (z.B. als NodeJS Entwickler) könnt ihr (neuerdings?!) mit dem Befehl
```shell
cargo add clap --features derive,cargo
```
Clap in euer Projekt einbinden.

#### src/main.rs
Nun ändern wir die Ausgabe unseres Programmes und fügen clap hinzu. Dazu öffnen wir die Datei `src/main.rs`
```Rust
fn main() {
    println!("Hello, world!");
}
```
Wie in der obigen Übersicht erwähnt erwartet der Rustcompiler, sollte man ihn nicht konfiguriert haben
immer eine Datei namens `main.rs` im Ordner `src`. In dieser Datei muss zwingend eine Funktion namens `main` existieren.

> Auch hier gibt es wieder Möglichkeiten dieses Verhalten zu ändern. Möchte man beispielsweise
> "baremetal" programmieren, d.h.: Sein Programm direkt auf der CPU ausführen lassen, möchte man einen
> Programmeinstieg mit dem ersten Byte und nicht mit einem Funktionsnamen.

##### Hallo Welt
Innerhalb der `main()` Funktion sehen wir die Zeile 
```Rust
    println!("Hello, world!");
```
Ausdrücke mit einem `!` am Ende ihres Namens sind in Rust keine Funktionen sondern Macros und werden zur
Compile-Zeit übersetzt. `println!` ist ein Makro zum ausgeben von Text auf der Kommandozeile.

Ändern wir nun "Hello, world!" in "Hallo Welt!" und führen den folgenden Befehl aus.
```shell
cargo run
```

Nun sollte sich etwas mehr in der Ausgabe getan haben: Der Compiler übersetzt nun auch Clap und seine Abhängigkeiten bevor 
als letzte Zeile unser "Hallo Welt!" erscheint.

> Möchte man die Abhängigkeiten seines Projektes analysieren kann der Befehl `cargo tree` einen
> Abhängigkeitsgraphen zeichnen.

