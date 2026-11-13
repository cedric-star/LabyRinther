# Bevy

## Ressourcen

- quasi ein Singleton
- mit `commands.insert_resource()` da erstellt man selber
- mit `commands.init_resource<>()` da wird Default aufgerufen

## States

- State des spiels (ingame, settings, splashmenu, ...)
- als enum definiert, ein enum element muss `#[default]` haben
- dann mit `init_state::<enum>()`
- für state wechseln:
  - `mut next: ResMut<NextState<GameState>>` in methode als eingabe
  - dann `next.set(...)`

### Scoped

- Componente `StateScoped` hinzufügen:
  `.enable_state_scoped_entities::<GameState>()`
- dann in einem `impl Bundle`: `StateScoped(enum)` setzten
- dieses Entity wird beim verlassen des states automatisch gelöscht

## Plugins

- für Plugin ein struct ohne alles anlegen `struct MyPlugin;`
- dann `impl Plugin for MyPlugin`
  - `fn build(&self, app: &mut App)`
  - app kann dann systeme hinzugefügt werden (oder resourcen,...)
- mit `.add_plugin()` den struct hinzufügen

# Rust
## Lifetimes
```rust
//lifetime checker weis nicht, ob rückgabe chunks aus vec<chunks> oder chunkpool stammt -> fehler
//lsg: 1. lifetimeannotation: 'a in funktion erstellt lifetime namens a
//2. 'a im vec: chunk referenzen leben mindesten 'a lang -> so lang wie funktion
//3. 'a im return (->): ergebnis hängt von 'a lifetime also vom vec ab
//-> lifetime sagt nicht wie lange etwas lebt, sonder beschreibt beziehungen
pub fn chunks_spawn_enemy<'a>(chunks: Vec<&'a Chunk>, pool: &ChunkPool, chunk_num: f32, p_pos: Vec3) -> Vec<&'a Chunk> {
    chunks
}
```
## Iterator
- iter()
  - leiht elemente aus -> gibt referenzen zurück (&T)
  - ursprüngliche collection ist aber noch dannach verwendbar
  - nur wenn gelesen wird und dannach weiterbenutzen
- iter_mut()
  - wie iter() aber gibt &mut T zurück
  - um elemente in collection zu ändern
- into_iter()
  - übernimmt ownership von collection 
  - gibt elemente selbst
  - ruft kein .clone() auf elementen auf
  - wenn elemente z.b. in andere sammlung verschoben werden sollen


# avian2d

## RigidBody

- types:
  - Dynamic -> für gegner (stoßen sich hin und her) -> hat eine masse
    - Mass(f32), CenterOfMass::new(f32, f32), AngularInertia (für angular masse)
  - Static -> für player (nur durch programmatische änderung bewegbar)
  - Kinematic -> für walls (garnicht bewegbar)
- kann sich auf 3 wege bewegen
  - transform with rigidbody in query ändern (sollte man nicht machen, wie ein
    teleport)
  - velocity ändern
    - LinearVelocity -> m/s in einen Vec2 eintragen (gibt richtung an) -> für
      daggerstich bspw
    - AngularVelocity -> radians/s als f32 (drehung um eigene achse) -> für
      schwertschwung bspws

### Forces

- hier nur bezug auf one time forces/impulses
- gelten für Dynamic RigidBodies
- types
  - force -> beschleunigung über zeit setzen
  - impuls -> velocity direkt auf wert gesetzt
  - accelaration -> unabhängig von masse (im gegensatz zu anderen types)
- on time -> Forces in query verwenden (ohne & oder &mut)
- dann for forces -> forces.apply... (vershiedene funktionen in Forces plugin)

### Domnianz

- RigidBody mit höherer Dominanz als ein anderer verhält sich dem gegenüber wir
  mit unendlicher masse
- rigidbody mit geringerer dominanz wird aber beeinflusst
- mit `Dominance(i8)`

### CCD

- continues collision detection
- für kleine schnelle objekte, die in einzelnen bewegungsschritten durch
  barrieren sich hindurchteleportieren würden (thunneling)
- SweptCcd
  - prüft auf weg alle möglichen kollisionen
  - spring zum anfang zurück, wenn kollision erkannt wird
  - zwei mode:
    - linear (SweepMode::Linear) halt gerade linie auf der geprüft wird
    - nonlinear (::NonLinear) überprüft zusätzlich rotierende bewegungen
- SpeculativeCcd
  - zieht spekultative margin um objekt
  - sagt kollisionen vorher
  - relevant für zwei schnell sich bewegende rigidbodies
  -

# Vec
- clamp() 
  - gib ein min unc max an, und der werte wird abgeschnitten, wenn er kleiner/größer als min max ist
