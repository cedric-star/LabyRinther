# Dev Blog 
-> https://gameblog.micedric.dpdns.org/

## Lizenz
Dieses Projekt steht unter einer modifizierten MIT-Lizenz (siehe LICENSE).
Private Nutzung und Weitergabe sind erlaubt, Namensnennung ist immer Pflicht.
Für kommerzielle Nutzung bitte vorher Kontakt aufnehmen: [cdrc.wnsch@gmail.com]



# next
- items aus secret, usbles und armor logik hinzufügen -> in itemdef ein doing felde als option
  - wenn none normales kack item
  - wenn some() -> dann kann es ein doing struct besitzen
  - doing hat optional ein weapon def, secret def, armordef, ...

- auf bevy 0.20 upgraden ): (aber wild eigentlich geile software uiuiui kiki)

- big picture: -> gegner spawnen uiuiui oder
- multiplayer mode:
  - welt als server oder singleplayer starten
  - dann welt welt beitreten oder singleplayer spielen.
  - ich will das man jede welt als singleplayer und mutliplayer spielen kann
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


