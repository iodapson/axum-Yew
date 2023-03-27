1. First mkdir `axum_yew_berline`

2. Created a `Cargo.toml` file

3. Added the first member crate; I added `frontend` first to hold Yew binary for frontend, and saved it. In the end, my `axum_yew_berline/Cargo.toml` looked like this;

```
[workspace]
members = [
    "frontend",
]
```

4. I created a new Cargo binary project called `frontend` and then ran command; `$ cargo build` from the workspace's root directory.

5. Added second member crate - `backend` to hold axum REST API backend, saved it. In the end, my `axum_yew_berline/Cargo.toml` looked like this;

```
[workspace]
members = [
    "frontend",
    "backend",
]
```

6. I created another Cargo binary project called `frontend` and then ran command; `$ cargo build` from the workspace's root directory.

7. I created a new directory called `slides` to hold the learning materials for both Yew and axum, and of course, this very document that you are now reading.

8. Ignored directory 'slides' by modifying my workspace' `Cargo.toml` file, and saved it. In the end the `Cargo.toml` file looked as follows;

```
[workspace]
members = [
    "frontend",
    "backend"
]
exclude = [
    "slides"
]
```

9. Once again, I built the cargo workspace.

10. Moving forward, I developed each workpace' member project independently. To run any particular member, I ran command;

```
$ cargo run -p <name-of-workspace-member>
```

11. To test the workspace as a whole, use command;

```
$ cargo test
```

However, if you wish to run tests for a particular workspace-member, run command;

```
$ cargo test -p <name-of-particular-workspace-member>
```

<b>N.B</b>:

To run the Yew app, from inside the `yew_frontend` directory, use trunk command:

```
$ trunk serve --proxy-backend=http://127.0.0.1:9080/return-json-data
```

To fire up the axum project, from inside the `axum_backend` directory, use Cargo command:

```
$ cargo run
```
