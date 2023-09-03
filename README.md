## Besida (¨бесіда¨)  ─  language for defining dialogue

### **Implementation**:
- **Besida**: struct that is initiated with path to dialogue file ([example](https://github.com/kshyr/besida/blob/dev/examples/basic.besida)) and parses it to **dialogue nodes**
- **Dialogue node**: holds speaker name and their speech (in form of **events** and in form of plain text)
- **Event**: enum that drives the design - pattern matching when iterating over events makes it very easy to execute actions in sequence as intended when writing dialogue in proposed format

Moving forward I want to add more events such as expressions/emotions of speaker, text speed control, text highlighting or styling in general. Also better ways to read actions when matching.

Here's an example with godot-rust where `event_tick` is triggered by letter display timer to simulate typing:
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
