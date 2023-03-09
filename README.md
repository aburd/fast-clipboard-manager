# fast-clipboard-manager

A clipboard manager designed for use in a tiled window manager, like i3 or sway.
Although not complete necessary to be used in that context, this manager in designed to be FAST and minimal and is aimed at users who never want to take their hands off the keyboard.
Inspired by projects like kickoff and rofi.

## Design Goals

- Fast
  - Able to confirm present clipboard selection entry in one second
    - Able to switch to recent entry in history in two seconds
    - Able to switch to older entry in history in ten seconds
- Secure
  - Any clipboard content is encrypted
  - Displaying clipboard content is always opt-in
- Minimal
  - The manager should blend into a variety of environments by default
- Configurable
  - Basic setup should be very easy to do on first startup
  - Finer grained controls should be offered through a config file
