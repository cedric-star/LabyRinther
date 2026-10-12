# next

- ALSNÄCHSTES: items und recipes als json schreiben und extern verwalten
- crafting links im bild: alle craftbaren items mit möglicher anzahl anzeigen:
  - ein klick -> eins
  - rechtsklick -> alle

# todo

- in welt können dinge platziert sein/werden (tilebasiert)
  - random fässer spawnen: linkcklick droppt kackitems
  - in rooms spawnen kisten verschiedener rarität
  - player kann permanent dinge platzieren

- animationen:
  - player walk
  - items in hand
  - intems benutzen
  - armor an player

- gegner spawnen in welt -> je nachdem wie weit region von spawn (0,0) entfernt
  ist -> infinity
  - gegner droppen loot -> gegner datenbank anlegen
  - attackpattern

- bosse
  - spawnen in größter roomgröße (mit kleiner warscheinlichkeit)
  - andere spawnbar durch gecraftete items

- thematische regionen
  - tile, gegner, ... assets ändern sich
  - anderer loot (bosse, gegner, truhen, ...)
  - bibiothek, horrorhaus, buschlabyrinth

- spieldaten serialisieren + speichern -> und wieder laden

# ideen

- item mit dem man sich als labyrinth wand verkleidet und stillstehen muss ->
  schutz vor gegner -> oder gegner die sich als wand verstecken, und erst
  revealen, wenn sie nah sind
- mana
  - items für dauerhafte managewinnung (2/s)
  - items die mana verbrauchen: 2% mehr damage, aber -2 mana die sekunde
