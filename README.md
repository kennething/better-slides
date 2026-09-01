<p align="center">
  <img src="./public/logo.svg" alt="Better Slides Logo" width="100" height="100">
</p>
<h1 align="center">Better Slides</h1>

Don't have a clicker thing for your presentation? Just use your phone silly!

Better Slides runs a Socket.IO WebSocket server locally on your machine, and
lets you connect to it from another device.

Built with [Tauri](https://tauri.app/), the app has a tiny footprint and works
on Windows and MacOS (Apple Silicon & Intel).

## Installation

1. Download the right installer for your system from the
   [releases page](https://github.com/kennething/better-slides/releases/latest/).

2. Ignore any warnings about the app being unsafe :shipit: (source: trust me
   bro)

   - On Windows, click "More Info" and then "Run Anyway"
   - On MacOS, go to System Preferences > Privacy & Security, scroll down, and
     click "Open Anyway"

3. Enable any accessibility permissions the app asks for :shipit:

   These are required for the app to function.

> \[!IMPORTANT\]
>
> Both the host and your other device must be on the same network, since the app
> exposes a local WebSocket server.
