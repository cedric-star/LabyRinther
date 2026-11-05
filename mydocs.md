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
- Componente `StateScoped` hinzufügen: `.enable_state_scoped_entities::<GameState>()`
- dann in einem `impl Bundle`: `StateScoped(enum)` setzten
- dieses Entity wird beim verlassen des states automatisch gelöscht

# Rust