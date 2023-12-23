# Über das Projekt
Ende November 2023 hat mein Professor in dem Modul "Kryptologie" ein Rätsel veröffentlicht:  
Gegeben ist ein n = p*q, wobei p und q Primzahlen sind.   
Wer ihm als erstes bis Ende Januar das p und q nennen kann, bekommt von ihm 100 Euro.

![img.png](img.png)

Die Sicherheit eines weitverbreiteten Verschlüsselungsalgorithmus (RSA) basiert genau darauf, dass die Lösung einer solchen Aufgabe sehr schwierig ist.

Aus dem Kontext der Vorlesung heraus kann man sich überlegen, dass die Zahl n mit der Software [ARIBAS](https://www.mathematik.uni-muenchen.de/~forster/sw/aribas.html) entwickelt wurde.
```
n := next_prime(random(10**100)) * next_prime(random(10**100)).
```


# Ansätze
## Faktorisieren
Ließe man eine Zahl p' im Bereich 10^99 <= p' < 10^100 laufen und würde Probedivisionen machen: n mod p' == 0, hätte man einen Aufwand von ca. 10^100 Probedivisionen. Im Vergleich: Die Anzahl der Atome im Universum liegt bei ca. 10^84.
Man würde also eine Ewigkeit warten müssen, bis eine Probedivision zum Erfolg führt.
Dies ist natürlich ein sehr naiver Ansatz und es gibt tatsächlich komplizierte Möglichkeiten, eine solch "kleine" Zahl anzugreifen. RSA Zahlen sind heutzutage daher weitaus größer.

## Zufälligkeit angreifen
In manchen Programmen wird ein schlechter Zufall genutzt, sodass man bspw. mit Kenntnis des Startzeitpunkts des Programms, die selben Zufallszahlen, also auch die selben Primzahlen ziehen kann.
Beispiel:
```c
srand(time(NULL)); // time(NULL) gibt die Anzahl der vergangenen Sekunden seit dem Jahr 1970 zurück.
```

Die Überlegung ist nun, ob ARIBAS auch zu solchen Programmen zählt.
Nachforschungen zeigen, dass ARIBAS zwar einen eigenen Zufallsalgorithmus bereitstellt, jedoch für den Seed auf die Berechnung
```c
srand(time(NULL));
aribas_seed = rand();
```
zurückgreift.
Ich möchte mich in dem Zuge auch bei Herrn Prof. Forster bedanken (der Entwickler von ARIBAS). Er hatte mir bei meinen Fragen bezüglich der Implementierung geholfen.

Da ich sicher nicht sagen kann, wann mein Professor sein Programm geöffnet hat (um das n zu berechnen), muss ich stattdessen versuchen den Zeitraum eingrenzen und alle Sekunden in diesem Zeitraum ausprobieren. Leider stellt sich auch das als Schwierigkeit heraus, da dieses Rätsel schon in den Jahren zuvor gestellt wurde und der Professor nicht sagen wollte, wann er das Rätsel das erste mal gestellt hat.
Ist der Bruteforce Algorithmus jedoch schnell genug, kann man einfach ein paar Jahre (jeweils im Zeitraum zwischen November und Dezember) durchtesten und den Rechner einen Tag durchlaufen lassen. Der Aufwand, um 5 Jahre durchzutesten, sollte in der Größenordnung 2^24 bzw. 10^8 liegen (Anzahl an Primzahlberechnungen "next_prime"). Das sind also nur ein paar Millionen Möglichkeiten (was sehr viel besser machbar ist).

Da mir ARIBAS selbst zu langsam war und keine Funktionen zu Paralleliserung von Berechnungen bereitstellt, habe ich mir kurzerhand dazu entschlossen, den Zufallsalgorithmus nachzuimplementieren.
Ich musste mich ein wenig in dem Source Code von Herrn Forster zurechtfinden. Schließlich hatte ich dann aber einen guten Überblick und konnte auch eigene Funktionen implementieren, sodass mir das Debugging einfacher viel. 

Da ich dafür zur Sprache Rust gehen wollte, weil ich diese einerseits cool finde und ich andererseits keine gute Bibliothek für Benutzung riesiger Integerzahen in C++ gefunden habe, musste ich also auch das srand() und rand() selbst implementieren.
Im ersten Versuch wollte ich in C den Code als Bibliothek an Rust weitergeben. Jedoch hat das nicht richtig geklappt.
Spätere Nachforschungen zeigten zudem, dass die Implementierung dieser Funktionen sehr von der Wahl des Betriebssystem und des Compilers abhängt. Für meine Zwecke war es essentiell, dass es mit GCC auf Windows kompiliert wird.
Mit dem Ansatz der Bibliothek hätte ich zudem das Problem, dass rand() einen globen Zustand hat, was für Parallelisierung nicht gerade von Vorteil ist.

Nun habe ich mich also dazu entschlossen, die Implementierung vollständig selbst umzusetzen. Die tatsächliche Implementierung zu finden stellte sich aber als große Herausforderung heraus. Ich habe mir den Source Code von glibc geholt und versucht nachzuvollziehen, wie dort die Implementierung umgesetzt wurde.
Nach vielem hin und her habe ich es dann endlich geschafft.
Sehr viel später stellte sich dann heraus, dass das die Implementierung auf Linux ist (ich habe aus Komfortgründen viel in der WSL Konsole gearbeitet) und mein Angriff somit im Grunde nie funktionieren konnte (außer als ich die Zufallszahl mit einem ARIBAS in Linux generiert habe).
Zum Glück konnte mir ein Kommilitone helfen und fand eine Python Implementierung der Windowsvariante.
Nach der Implementierung dieser Version konnte ich quasi den Angriff starten.

Zuvor jedoch, als ich noch mit der Linuxvariante gearbeitet habe, implementierte ich dann nun auch den Zufallsalgorithmus von ARIBAS selbst. Auch dort hatte ich hier und da Probleme, weil ich die Implementierung ein wenig anders und "moderner" lösen wollte, als es im ARIBAS Code gemacht wurde.

Nach vielem hin und her funktionieren nun beide Zufallsgeneratoren zuverlässig. Der hier zur Verfügung gestellte Code bietet eine Variante für Linux und Windows an.

Ich habe mich am Code für den Bruteforce Algorithmus dann schließlich auch ziemlich lange aufgehalten, weil ich eine Progressbar haben wollte. Am Ende entschied ich mich dann doch dagegen, um mehr Informationen über den Verlauf sehen zu können (und weil ich nicht noch mehr Zeit verlieren wollte).
Nach dem Festlegen einer Zeitspanne, wollte ich das Programm nun über Nacht laufen lassen... Um nach 3 Minuten festzustellen, dass es bereits ein Ergebnis gibt: Das Rätsel war gelöst.

Am nächsten Tag tauschte ich mich noch mit meinem Kommilitonen aus. Wir sind darauf gekommen, dass die Wahl einer Zeitspanne ziemlich unrelevant war, da die Implementierung von rand() in Windows weniger als 2^16 Ausgaben haben kann.

In der finalen Version werden nun einfach alle Möglichkeiten von 0 bis 0x7fff ausprobiert. Erfahrungsgemäß dauert ein Angriff bis zu zwei Minuten.

# Selbst ausprobieren
In der Datei main.rs ist die Zahl in der Zeile 33 das n.
Nach der Installation von Rust kann man dann den Befehl
```shell
cargo run --release
```
ausführen und auf das Ergebnis warten.