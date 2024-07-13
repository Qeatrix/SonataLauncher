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
- [ ] Implement **natives** folder handling
- [ ] Separate endpoint for instance launching
- [ ] Code refactoring in `instance/mod.rs`
- [ ] Endpoint for sending of installed instances
	- [ ] Installed instances storing implementation in JSON
- [ ] Endpoint for sending required authenticated account
	- [ ] Storing authenticated accounts
#### Frontend
- [ ] Instance download manager with WebSockets integration
- [ ] Tasks widget on headerbar
	- [ ] Storage of minimized windows
- [ ] Retrieving installed instances
	- [ ] Reusable grid display component
- [ ] Account authentication

## Application Architecture
![Application Architecture Map](./Application%20Architecture.png)
