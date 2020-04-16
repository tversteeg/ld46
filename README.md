<h1 align="center">
	<a href="https://tversteeg.nl/ld46/">ld46</a><br/>
	<a href="https://github.com/tversteeg/ld46/releases">
		<img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/linux.svg" width="18" height="18" />
		<img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/apple.svg" width="18" height="18" />
		<img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/windows.svg" width="18" height="18" />
	</a>
	<a href="https://tversteeg.nl/ld46/">
		<img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/mozillafirefox.svg" width="18" height="18" />
		<img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/googlechrome.svg" width="18" height="18" />
		<img src="https://cdn.rawgit.com/simple-icons/simple-icons/develop/icons/safari.svg" width="18" height="18" />
	</a>
</h1>
<p align="center">
	A game, you can play!
</p>
	
<p align="center">
	<a href="https://github.com/tversteeg/ld46/actions"><img src="https://github.com/tversteeg/ld46/workflows/CI/badge.svg" alt="CI"/></a>
	<br/>
</p>

## Play

Play it directly in your browser [**here**](https://tversteeg.nl/ld46)!

Or download the executable file from the [Releases](https://github.com/tversteeg/ld46/releases) tab and execute it.

> You might have to change the permissions on Linux with `chmod u+x ld46-*`

## Instructions

TODO

## Build

You will need an up-to-date [Rust](https://rustup.rs/) setup.

### Linux Dependencies

To build it on linux you will need the X11, OpenGL & Alsa development libraries:

```bash
sudo apt install libasound2-dev libx11-dev libxi-dev libgl1-mesa-dev
```

## Run

### Native

You just need to run the following to compile & run the game after you've installed the dependencies:

```bash
cargo run --release
```

### WASM

Add the `wasm32` target to Rust, build it with that target & copy it to the root:

```bash
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/ld46.wasm .
```

Now we have to host the website:

```bash
cargo install basic-http-server
basic-http-server .
```
