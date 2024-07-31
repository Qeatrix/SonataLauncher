# Sonata Launcher
The next generation Minecraft Launcher
## The project is under heavy development
### Build the project

Fetch the source code
```
git clone https://github.com/Qeatrix/SonataLauncher.git
```

Build frontend
```
cd app && npm run build
```

Build ActionHandler
```
cd action_handler && cargo run build
```

## Project Todo
#### Action Handler
- [ ] Logger implementation
- [x] Different Java versions downloader
  - [x] Comapre every Java downloaded path
  - [ ] Endpoint for Java download
  - [ ] WebSockets integration
- [x] Home directory handling
- [ ] Refactor WebSocket messages types
- [ ] Implement **natives** folder handling
- [ ] Separate endpoint for instance launching
- [ ] Code refactoring in `instance/mod.rs`
- [ ] Endpoint for sending installed instances
	- [x] Instance directory initialization
 		- [x] Refactor `init_instance_dir` function
	- [x] Installed instances storing implementation in JSON
- [ ] Endpoint for sending required authenticated account
	- [ ] Storing authenticated accounts
#### Frontend
- [ ] Refactor WebSocket messages types
- [x] Instance download manager with WebSockets integration
  - [ ] Switching between percentage and elapsed downloading time
	- [ ] Correct `Cancel` button handling
- [ ] Tasks widget on headerbar
	- [ ] Storage of minimized windows
- [ ] Retrieving installed instances
	- [ ] Reusable grid display component
- [ ] Account authentication
- [ ] Refactor localization store
- [ ] Migrate from Electron to Tauri
    - [ ] Verify correct rendering of the interface in webview on different platforms
        - [ ] Linux
        - [x] macOS
        - [ ] Windows

## Application Architecture
![Application Architecture Map](./Application%20Architecture.png)
