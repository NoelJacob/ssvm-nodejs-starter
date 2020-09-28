# WASM Demo - Pattern Generator

1

1       1

1       1       1

1       1       1       1

When count=4

Use localhost:3000?count=**N** to generate this pattern

## Getting started

![Build and test](https://github.com/second-state/ssvm-nodejs-starter/workflows/Build%20and%20test/badge.svg)

[Fork this project](https://github.com/second-state/ssvm-nodejs-starter/fork) to create your own Rust functions in Node.js. [Learn more](https://www.secondstate.io/articles/getting-started-rust-nodejs-vscode/)

* The Rust functions are in the `src` directory. You can put high performance workload into Rust functions.
* The JavaScript functions are in the `node` directory and they can access the Rust functions.
* Use the `node node/app.js` command to run the application in Node.js.


## Use Docker to build and run

```
$ docker pull secondstate/ssvm-nodejs-starter:v1
$ docker run -p 3000:3000 --rm -it -v $(pwd):/app secondstate/ssvm-nodejs-starter:v1
(docker) # cd /app
(docker) # ssvmup build
(docker) # node node/app.js
```

From a second terminal window, you can test the local server.

```
$ curl http://localhost:3000/?name=SSVM
hello SSVM
```


## Use VSCode Codespace

<p>
    <a href="https://online.visualstudio.com/environments/new?name=Rust%20and%20WebAssembly%20in%20Node.js&repo=second-state/ssvm-nodejs-starter">
        <img src="https://img.shields.io/endpoint?style=social&url=https%3A%2F%2Faka.ms%2Fvso-badge">
    </a>
</p>

![SSVM](https://github.com/second-state/blog/blob/master/static/images/SSVM-edited-without-music.gif?raw=true)

This project template works with the VS Codespaces online IDE! Code, build, and run directly from inside the browser. No software download or install needed! Check out the [high-res screencast](https://youtu.be/j85cbNsciOs).

> VS Codespaces runs entirely in your browser and costs around $1 per work day. It is cheaper than a cup of coffee in the office. Alternatively, use locally installed VSCode and Docker, and [launch the IDE with your remote git repository](https://code.visualstudio.com/remote-tutorials/containers/getting-started).

1 First, open the [VS Codespaces](https://online.visualstudio.com/) web site and login with your Azure account. You can get a [free Azure account](https://azure.microsoft.com/en-us/free/).

2 Next, create a new Codespace. Put your forked repository into the Git Repository field.

![Create a new Codespace](docs/img/vscode_create.png)

3 Then open the `src/lib.rs`, `node/app.js` and `Cargo.toml` files and see how the Node.js express app calls the Rust function to say hello.

![Code in Codespace](docs/img/vscode_code.png)

4 Click on the Run button on the left panel, and then the Launch Program at the top to build and run the application.

![Build and run](docs/img/vscode_run.png)

The Terminal window at the bottom shows the build progress. It builds the Rust program, and then launches the Node.js app.

![Build](docs/img/vscode_build.png)

The Debug window shows the Node.js server running and waiting for web requests.

![Debug](docs/img/vscode_debug.png)

5 Now, you have two choices. You could use the proxy link for `127.0.0.1:3000` to access the running server in a browser.

![Browser link](docs/img/vscode_port.png)

Or, you could open another terminal window in the IDE via the `Terminal -> New Terminal` menu.

![Open Terminal](docs/img/vscode_terminal.png)

From the terminal window, you can test the local server.

```
$ curl http://127.0.0.1:3000/?name=SSVM
hello SSVM
```

