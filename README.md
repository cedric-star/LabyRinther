# todo

- inventar -> items anzegigen, die man hat: 4 + hotbar = 5 bars am anfange, up
  to 7
- inventar -> rüstungsslots
- inventar -> items draggable machen
- inventar -> passive slots -> für passive effekte: 4, bis zu 10 (nimmt dann
  eine bar ein)
- crafting links im bild: alle craftbaren items mit möglicher anzahl anzeigen:
  - ein klick -> eins
  - rechtsklick -> alle

- hp, armor, mana bar
  - mana und health sind normal n/100 -> aber durch items max size erhöbar
  - armor sind 0-60 einheiten
  - armor reduziert eingehenden schaden um 0 - 60%

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

- mana
  - items für dauerhafte managewinnung (2/s)
  - items die mana verbrauchen: 2% mehr damage, aber -2 mana die sekunde
