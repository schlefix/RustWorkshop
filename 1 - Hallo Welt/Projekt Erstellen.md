# Projekt initialisieren
### Wichtig am Rande:
Rust bevorzugt lower_snake_case! Wird das nicht eingehalten haut einem der Compiler Warnungen um die Ohren
und kann (im schlimmsten Fall, wenn er Streng eingestellt ist) nicht compilieren.
Dieses Verhalten kann aber mit einer Annotation deaktiviert werden ;)


### Erstellen eines neuen Projektes
Wir bewegen uns in den Ordner, in dem wir das Projekt anlegen wollen und führen folgenden Befehl aus

``` 
cargo new name_generator 
```
Anschließend erstellt der Package-Manager "Cargo" alle nötigen Dateien um ein Ausführbares Program (in unserem Fall
für unseren "name_generator") zu erstellen
Der Zusatz`--lib` hingegen erstellt nur die Nötigen Dateien um eine Bibliothek anzulegen, welche in anderen Projekten verwendet werden kann.

Navigieren wir nun in den neu erstellten Ordner unseres Projektes und führen
```
> cd name_generator
> cargo run
```
Nach einer kurzen Zeit der erstellung durch den Compiler sollte nun folgender Text in der Konsole erscheinen
```
<   Compiling name_generator v0.1.0 (C:\Users\schle\IdeaProjects\Rust Workshop\1 - Hallo Welt\name_generator)
<     Finished dev [unoptimized + debuginfo] target(s) in 0.33s
<      Running `target\debug\name_generator.exe`
< Hello, world!
```
Die ersten 3 Zeilen sind der Compiler, welcher ein paar Informationen zu seiner Arbeit liefert.
Das "Hello, world!" ist schon die Ausgabe unseres Programmes!

### Anpassen des Sourcecodes
Da wir dort nicht `Hello, world!` stehen haben wollen, sondern etwas sinnvolles, schauen wir uns hier 
die, von Cargo erstellten Dateien:

```
name_generator/             # Projektordner
    src/                    # Haupt-Ordner für Quellcode
        main.rs             # Haupt Programm Einstieg (zu den verschiedenen "festen" Dateinamen später mehr)
    target/                 # Kompilat/Output-Ordner nach cargo build / cargo run
    Cargo.lock              # NICHT EDITIEREN: Hier "merkt" sich Cargo den Zustand des Projektes
    Cargo.toml              # Projektkonfiguration, ähnlich zu package.json bei nodeJS oder requirements.txt
```