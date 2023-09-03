## Besida (¨бесіда¨) ─ language for defining dialogue

### Usage

Your text file - let's call it `the_part_where_he_kills_you.txt`:

```
--- Chapter 9 Transition ---

GLaDOS:
    Well, this is the part where he kills us.

Wheatly:
    Hello! This is the part where I kill you!

[announce_next_chapter]
[unlock_achievement]
```

Initialization:

```rust
use besida::Besida;
use std::path::Path;

// ...

let dialogue_file_path = Path::new("the_part_where_he_kills_you.txt");
let besida = Besida::new(dialogue_file_path);
```

Somewhere in loop:

```rust
{
    let Some(node) = besida.get_node_mut() else { return };
    let Some(event) = node.get_event() else { return };

    match event {
        Event::PrintChar(char) => {
            // append character for typed out text effect
        },
        Event::Action(action) => {
            match actions.as_str() {
                "announce_next_chapter" => {
                    // call UI to announce chapter / animate transition
                },
                "unlock_achievement" => {
                    // call achievement system to set specific achievement unlocked
                }
            }
        }
        _ => {},
    }

    node.next_event();
}
```

---

### **Implementation**:

- **Besida**: struct that is initiated with path to dialogue file ([example](https://github.com/kshyr/besida/blob/dev/examples/basic.besida)) and parses it to **dialogue nodes**
- **Dialogue node**: holds speaker name and their speech (in form of **events** and in form of plain text)
- **Event**: enum that drives the design - pattern matching when iterating over events makes it very easy to execute actions in sequence as intended when writing dialogue in proposed format

Here's an example with `godot-rust` where `event_tick` is triggered by letter display timerevery n milliseconds to simulate typing:

```rust
    #[func]
    fn event_tick(&mut self) {
        let Some(node) = self.besida.get_node_mut() else { return };
        let Some(event) = node.get_event() else { return };

        let mut dialogue_box = self
            .base
            .get_parent()
            .unwrap()
            .get_node_as::<DialogueBox>("DialogueBox");

        match event {
            Event::PrintChar(char) => {
                let speech = dialogue_box.bind().get_speech();
                let new_speech = format!("{}{}", speech, char);

                dialogue_box.bind_mut().set_speech(new_speech.into());
            }
            _ => {},
        }

        node.next_event();
    }

```

Moving forward I want to add more events such as expressions/emotions of speaker, text speed control, text highlighting or styling in general. Also better ways to read actions when matching.
