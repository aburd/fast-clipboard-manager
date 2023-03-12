# fast-clipboard-manager

A clipboard manager designed for use in a fast window manager, like i3 or sway.

Aimed at users who never want to take their hands off the keyboard, this manager in designed to be FAST, minimal.

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

![Screenshot as of 2023-03-12](https://github.com/aburd/fast-clipboard-manager/blob/main/resources/demo-2023-03-12%2021-10.gif?raw=true)

## Todo List

- [x] Make basic config
- [x] Persist selections to file
- [x] Encypypt Entries
- [ ] Accessibility (escape to close, click off screen to close, etc)
- [ ] Onboarding
- [ ] Key config
- [ ] Style config
- [ ] Don't show whole selection on screen

