# Sheet Sharp
You can now write and play your own music in Sheet Sharp (`.ss`),  a music-like language. Sound is played through rodio.

# Features
Create a new file with a .ss extension. Sheet Sharp needs three keywords `BPM`, `MusicSheet`, and `Play`.

`BPM` is a keyword. A BPM of 120 can be set like this
```
BPM 120
```

`MusicSheet` defines what notes will be played. `Notes` can have two parameters, a *pitch* (the note itself) and a *duration* (Beats measured in quarter-notes). Each note needs to be separated by a comma. 
A sheet-sharp file that plays a singular A4 note can be defined in a music sheet like so:
```
MusicSheet = {
  Note(A4, 1)
}
``` 

Finally, call the `Play` keyword to hear the music.
```
Play
```

A complete example would look like this:
```
BPM 160

MusicSheet = {
  Note(E5, 1), Note(D5, 1), Note(C5, 1), Note(D5, 1), Note(E5, 1), Note(E5, 1), Note(E5, 2)
}

Play
```

# Configuration
Download and open to music_sheets folder. Write your music in a new file with extension `.ss`. Make sure you are in the root directory. 
```
cargo r
```
Run `cargo r` or `cargo run` to start.

Enter music sheet file path
```
Enter file path: music_sheets/<example.ms>
```

# To-Do
- WIP: Transposition
- WIP: Chords
- WIP: Left hand / right hand / both hands
- WIP: Optional Note / Chord parameters
- WIP: Import instruments
- WIP: Save music as mp3 or wav
