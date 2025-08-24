# Enigma-Maschinen

**Sprachen:** [English](README.md) | Deutsch

Eine historisch genaue Implementierung des Enigma-Maschinenverschlüsselungssystems aus dem Zweiten Weltkrieg, geschrieben in Rust. Dieser Simulator reproduziert originalgetreu die mechanischen Abläufe der ursprünglichen Enigma-Maschine, einschließlich der Rotorschaltmechanik, Steckbrettkonfigurationen und Reflektorfunktionalität.

## Überblick

Die Enigma-Maschine war ein Chiffriergerät, das während des Zweiten Weltkriegs von Nazi-Deutschland ausgiebig zur Verschlüsselung und Entschlüsselung geheimer Nachrichten verwendet wurde. Diese Implementierung bietet eine vollständige Simulation der kryptographischen Mechanismen der Maschine und eignet sich für Bildungszwecke, historische Forschung und kryptographische Analyse.

### Hauptmerkmale

- **Authentische Rotormechanik**: Drei-Rotor-System mit historisch genauer Schaltmechanik einschließlich Doppelschaltung
- **Konfigurierbares Steckbrett**: Unterstützung für bidirektionale Zeichenvertauschung durch TOML-Konfiguration
- **Zufällige Reflektorgenerierung**: Automatische Generierung gültiger Reflektorverdrahtungen
- **Serialisierte Rotorzustände**: Binäre Serialisierung von Tagesschlüssel-Konfigurationen
- **Kommandozeilenschnittstelle**: Vollständige CLI mit Generierungs- und Verschlüsselungsfunktionen
- **Fehlerbehandlung**: Umfassende Fehlerverwaltung für ungültige Eingaben und Konfigurationen

## Technische Architektur

### Rotorsystem

Die Implementierung verwendet drei Rotoren mit folgenden Eigenschaften:

- **Rotor 1 (Rechts)**: Schneller Rotor mit Übertragskerbe bei Position 16
- **Rotor 2 (Mitte)**: Mittlerer Rotor mit Übertragskerbe bei Position 4
- **Rotor 3 (Links)**: Langsamer Rotor mit Übertragskerbe bei Position 21

Jeder Rotor implementiert sowohl Vorwärts- als auch Rückwärtsverschlüsselungspfade und simuliert den elektrischen Signalweg durch die physische Maschine.

### Steckbrettkonfiguration

Das Steckbrett ermöglicht die Konfiguration von bis zu 13 bidirektionalen Zeichenpaaren über TOML-Dateien. Diese Funktion erhöht den Schlüsselraum erheblich und war eine entscheidende Sicherheitsverbesserung in späteren Enigma-Varianten.

### Alphabetunterstützung

Diese Implementierung unterstützt ein 53-Zeichen-Alphabet, bestehend aus:
- Kleinbuchstaben (a-z)
- Großbuchstaben (A-Z)
- Leerzeichen

## Installation

### Voraussetzungen

- Rust 1.70 oder höher
- Cargo-Paketmanager

### Abhängigkeiten

```toml
[dependencies]
clap = "4.0"
rand = "0.8"
serde = { version = "1.0", features = ["derive"] }
bincode = "1.3"
toml = "0.8"
```

### Erstellen

```bash
git clone https://github.com/naseridev/enigma.git
cd enigma
cargo build --release
```

## Verwendung

### Ersteinrichtung

Bevor Sie die Enigma-Maschine verwenden können, müssen Sie zunächst die erforderlichen Konfigurationsdateien generieren:

#### 1. Rotorkonfiguration generieren (Tagesschlüssel)

Generieren Sie eine neue Tagesschlüsseldatei mit randomisierten Rotorverdrahtungen:

```bash
# Generierung mit Standarddateiname
./enigma --generate

# Generierung mit benutzerdefiniertem Dateinamen
./enigma --generate --rotor-file mein_tagesschluessel.enigma

# Generierung mit spezifischem Ausgabepfad
./enigma -g -r /pfad/zu/schluesseln/enigma_20241125.key
```

#### 2. Steckbrettkonfiguration generieren

Erstellen Sie eine Steckbrett-Konfigurationsvorlage:

```bash
# Generierung mit Standarddateiname
./enigma --generate-plugboard

# Generierung mit benutzerdefiniertem Dateinamen
./enigma --generate-plugboard --plugboard-file mein_steckbrett.toml

# Generierung mit spezifischem Ausgabepfad
./enigma -p -b /pfad/zu/configs/station_steckbrett.toml
```

Nach der Generierung bearbeiten Sie die Steckbrettdatei, um Ihre gewünschten Zeichenpaare hinzuzufügen:

```toml
pairs = [
    "ab",  # Ordnet 'a' zu 'b' und 'b' zu 'a' zu
    "CD",  # Ordnet 'C' zu 'D' und 'D' zu 'C' zu
    "ef",  # Ordnet 'e' zu 'f' und 'f' zu 'e' zu
    "XY",  # Ordnet 'X' zu 'Y' und 'Y' zu 'X' zu
    "z ",  # Ordnet 'z' zu Leerzeichen und Leerzeichen zu 'z' zu
]
```

### Grundlegende Operationen

#### Einfache Verschlüsselung/Entschlüsselung

Grundlegende Verschlüsselung mit Standardeinstellungen (Rotorpositionen "aaa"):

```bash
./enigma "HALLO WELT"
# Ausgabe: verschlüsselte Nachricht

./enigma "hallo welt"
# Ausgabe: verschlüsselte Nachricht (Groß-/Kleinschreibung beibehalten)

./enigma "Der schnelle braune Fuchs springt über den faulen Hund"
# Ausgabe: verschlüsselte Nachricht mit beibehaltenen Leerzeichen
```

#### Verwendung benutzerdefinierter Rotorpositionen

Anfängliche Rotorpositionen für Verschlüsselung festlegen:

```bash
# Rotorpositionen auf A, B, C setzen
./enigma --start-positions "ABC" "GEHEIME NACHRICHT"

# Rotorpositionen auf X, Y, Z setzen
./enigma -s "XYZ" "STRENG GEHEIM"

# Kleingeschriebene Positionen verwenden
./enigma --start-positions "xyz" "klassifizierte information"

# Gemischte Positionen
./enigma -s "AbC" "gemischte nachricht"
```

### Erweiterte Konfiguration

#### Verwendung benutzerdefinierter Konfigurationsdateien

Benutzerdefinierte Rotor- und Steckbrettdateien angeben:

```bash
# Nur benutzerdefinierte Rotordatei verwenden
./enigma --rotor-file benutzer_rotoren.enigma "NACHRICHT"

# Nur benutzerdefinierte Steckbrettdatei verwenden
./enigma --plugboard-file benutzer_steckbrett.toml "NACHRICHT"

# Beide benutzerdefinierte Dateien verwenden
./enigma --rotor-file tagesschluessel_nov25.enigma --plugboard-file station_alpha.toml "NACHRICHT"

# Abgekürzte Flags verwenden
./enigma -r mein_schluessel.enigma -b mein_steckbrett.toml -s "DEF" "VERSCHLÜSSELTER TEXT"
```

#### Vollständige Befehlsbeispiele

Vollständige Verschlüsselung mit allen angegebenen Parametern:

```bash
# Morgendliche Übertragungseinrichtung
./enigma \
  --rotor-file schlussel/morgen_schluessel.enigma \
  --plugboard-file configs/station_1_steckbrett.toml \
  --start-positions "MRG" \
  "Wetterbericht klarer Himmel erwartet"

# Abendliche Übertragungseinrichtung
./enigma \
  -r schlussel/abend_schluessel.enigma \
  -b configs/station_2_steckbrett.toml \
  -s "ABD" \
  "Mission erfolgreich Rueckkehr zur Basis"

# Notfallübertragung (minimales Steckbrett)
./enigma \
  --rotor-file notfall_schluessel.enigma \
  --start-positions "SOS" \
  "Notfallevakuierung sofort erforderlich"
```

### Entschlüsselungsvorgang

Die Enigma-Maschine ist symmetrisch - die Entschlüsselung verwendet denselben Prozess wie die Verschlüsselung:

```bash
# Ursprüngliche Nachricht
./enigma -s "ABC" "ANGRIFF BEI MORGENDÄMMERUNG"
# Beispielausgabe: "FKPQM ZU XWVNKLMPO"

# Entschlüsseln (gleiche Einstellungen)
./enigma -s "ABC" "FKPQM ZU XWVNKLMPO"
# Ausgabe: "ANGRIFF BEI MORGENDÄMMERUNG"
```

### Stapeloperationen

#### Mehrere Nachrichten mit gleichen Einstellungen

```bash
# Mehrere Nachrichten verschlüsseln unter Beibehaltung des Rotorzustands
./enigma -s "KEY" "ERSTE NACHRICHT" > verschluesselt1.txt
./enigma -s "KEY" "ZWEITE NACHRICHT" > verschluesselt2.txt
./enigma -s "KEY" "DRITTE NACHRICHT" > verschluesselt3.txt
```

#### Verschiedene Tagesschlüssel

```bash
# Montags-Nachrichten
./enigma -r schlussel/montag.enigma -s "MON" "Morgenbesprechung abgeschlossen"

# Dienstags-Nachrichten
./enigma -r schlussel/dienstag.enigma -s "DIE" "Nachmittags-Patrouille geplant"

# Mittwochs-Nachrichten
./enigma -r schlussel/mittwoch.enigma -s "MIT" "Abendbericht eingereicht"
```

### Fehlerbehebung bei häufigen Problemen

#### Datei-nicht-gefunden-Fehler

```bash
# Prüfen, ob Rotordatei existiert
ls -la tagesschluessel.enigma

# Generieren falls fehlend
./enigma --generate

# Absoluten Pfad verwenden falls nötig
./enigma --rotor-file /vollstaendiger/pfad/zu/tagesschluessel.enigma "NACHRICHT"
```

#### Ungültige-Zeichen-Fehler

```bash
# Nur gültige Zeichen
./enigma "GUELTIGE NACHRICHT mit Leerzeichen"

# Ungültige Zeichen verursachen Fehler
./enigma "Ungültig: 123 !@#"  # Zahlen und Symbole nicht unterstützt
```

#### Ungültige-Rotorposition-Fehler

```bash
# Gültige Positionen (müssen genau 3 Zeichen aus Alphabet + Leerzeichen sein)
./enigma -s "ABC" "nachricht"  # Gültig
./enigma -s "xyz" "nachricht"  # Gültig
./enigma -s "A z" "nachricht"  # Gültig (enthält Leerzeichen)

# Ungültige Positionen
./enigma -s "AB" "nachricht"   # Fehler: zu kurz
./enigma -s "ABCD" "nachricht" # Fehler: zu lang
./enigma -s "AB1" "nachricht"  # Fehler: ungültiges Zeichen
```

### Kommandozeilen-Argumente-Referenz

#### Erforderliche Argumente
- `<nachricht>`: Zu verschlüsselnde/entschlüsselnde Nachricht (erforderlich außer bei Verwendung von Generierungsflags)

#### Optionale Argumente
- `-g, --generate`: Neue Rotorkonfigurationsdatei generieren
- `-p, --generate-plugboard`: Steckbrett-Konfigurationsvorlage generieren
- `-r, --rotor-file <DATEI>`: Pfad zur Rotorkonfigurationsdatei (Standard: `./daily_key.enigma`)
- `-b, --plugboard-file <DATEI>`: Pfad zur Steckbrett-Konfigurationsdatei (Standard: `./plugboard.toml`)
- `-s, --start-positions <POSITIONEN>`: Anfängliche Rotorpositionen als genau 3 Zeichen (Standard: `"aaa"`)

#### Hilfe und Version
```bash
./enigma --help     # Hilfeinformationen anzeigen
./enigma --version  # Versionsinformationen anzeigen
```

### Leistungsüberlegungen

Für optimale Leistung mit großen Nachrichten:

```bash
# Große Dateien weiterleiten
cat grosse_nachricht.txt | xargs ./enigma -s "KEY"

# Mehrere Dateien verarbeiten
for datei in nachrichten/*.txt; do
    ./enigma -s "$(date +%j)" "$(cat "$datei")" > "verschluesselt/$(basename "$datei")"
done
```

## Konfigurationsdateien

### Rotorkonfiguration

Rotorkonfigurationen werden als Binärdateien mit bincode-Serialisierung gespeichert. Die Struktur enthält drei randomisierte Substitutionsalphabete, die sicherstellen, dass keine Fixpunkte existieren (eine Sicherheitsanforderung).

### Steckbrettkonfiguration

Steckbretteinstellungen verwenden das TOML-Format:

```toml
# Enigma-Steckbrett-Konfiguration
pairs = [
    "ab",  # a <-> b
    "CD",  # C <-> D
    "X ",  # X <-> Leerzeichen
]
```

## Sicherheitsüberlegungen

### Historischer Kontext

Obwohl diese Implementierung historisch genau ist, wurde die Enigma-Chiffre während des Zweiten Weltkriegs durch verschiedene kryptanalytische Techniken gebrochen. Moderne Sicherheitsanwendungen sollten sich nicht auf Enigma-basierte Verschlüsselung verlassen.

### Kryptographische Schwächen

- **Keine Selbstverschlüsselung**: Zeichen können nicht zu sich selbst verschlüsselt werden
- **Vorhersagbare Rotorbewegung**: Regelmäßige Schaltmuster können ausgenutzt werden
- **Begrenzter Schlüsselraum**: Trotz scheinbarer Komplexität ist der effektive Schlüsselraum kleiner als moderne Standards
- **Häufigkeitsanalyse-Anfälligkeit**: Lange Nachrichten bleiben für statistische Angriffe anfällig

## Bildungsanwendungen

Dieser Simulator dient mehreren Bildungszwecken:

### Kryptographie-Bildung

- Demonstriert historische symmetrische Verschlüsselungstechniken
- Veranschaulicht die Bedeutung von Schlüsselverwaltung und -verteilung
- Zeigt die Entwicklung von mechanischen zu elektronischen kryptographischen Systemen

### Informatik-Pädagogik

- Beispiel für komplexe Zustandsautomaten-Implementierung
- Demonstriert Serialisierung und Konfigurationsverwaltung
- Veranschaulicht Kommandozeilen-Anwendungsdesignmuster

### Historische Forschung

- Bietet genaue Simulation für das Studium der Kommunikation im Zweiten Weltkrieg
- Ermöglicht die Analyse historischer Abfänge und Entschlüsselungen
- Unterstützt das Verständnis von Codeknacking-Methodologien

## Implementierungsdetails

### Rotorschalt-Algorithmus

Der Schaltmechanismus implementiert die "Doppelschaltungs"-Anomalie, die in tatsächlichen Enigma-Maschinen vorhanden war, bei der der mittlere Rotor bei aufeinanderfolgenden Tastendrücken schaltet, wenn er mit seiner Übertragskerbenposition ausgerichtet ist.

### Reflektorgenerierung

Der Reflektor wird zufällig generiert, wobei sichergestellt wird, dass jedes Zeichen genau einem anderen Zeichen zugeordnet wird, ohne dass ein Zeichen sich selbst zugeordnet wird.

### Fehlerbehandlung

Umfassende Fehlerbehandlung deckt ab:
- Ungültige Rotorpositionen
- Fehlerhaft formatierte Nachrichten mit nicht unterstützten Zeichen
- Datei-I/O-Fehler
- Serialisierungs-/Deserialisierungsfehler
- Ungültige Steckbrettkonfigurationen

## Leistungsmerkmale

- **Speicherverbrauch**: Minimale Heap-Allokation während des Betriebs
- **Verarbeitungsgeschwindigkeit**: Lineare Zeitkomplexität relativ zur Nachrichtenlänge
- **Datei-I/O**: Effiziente binäre Serialisierung für Rotorzustände
- **Konfigurationsladen**: TOML-Parsing mit Validierung

## Mitwirken

Beiträge sind willkommen für:

- Zusätzliche Rotorkonfigurationen basierend auf historischen Varianten
- Verbesserte Fehlermeldungen und Benutzererfahrungsverbesserungen
- Leistungsoptimierungen
- Zusätzliche Alphabetunterstützung
- Verbesserungen der historischen Genauigkeit