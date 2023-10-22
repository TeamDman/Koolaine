- Text generation server -- Gradio interface; WSL start.sh
- GLaDOS TTS server -- Git repo; python import
- Microphone capture server -- python import
- Voice transcription server -- whisperx

Some servers are easiest to start by just running wt.exe to start their respective startup script.
Some servers are easiest to think of as a small block of python code invoking a critical method.

For each server, we must know:
- how to start it
- how to check if it is running
- how to stop it
- how to get its timestamped logs
- what queues it depends on

We must facilitate queues:
- notification of new entry
- delete an entry
- publish an entry

---

Next steps:

Create a Rust program using the Ratatui library for presentation.
The TUI should be able to:
- start a server
- stop a server
- view logs
- view status
- view dependencies
- view queues
- view queue contents -- timestamp, author service, message body
- view queue listeners services

Immediate next steps:
1. cargo new project
2. add Ratatui dependency
3. create a TUI with a single screen
4. create tabbed list -- tab for each server -- that shows different message for each server when focused