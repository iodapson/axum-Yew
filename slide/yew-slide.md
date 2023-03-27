### Yew 0.20.0

( https://yew.rs )

#### What is Yew?

Yew is a modern Rust framework for creating multi-threaded front-end applications that are reliable and efficient using WebAssembly.

It used to be the case that ancient UI were procedural in nature. This means you had to wait until an event/user-action happens or a result is returned before another action/event occurs in a step by step order. You could not do something else unless an anticipated event happens or an anticipated result is received.
Modern UI (nowadays) are declarative in nature. You describe what should happen when an event happens, but you do not have to wait upon said event in order to move on with your program's execution flow. It is asynchronous. It is reactive.
It is a declarative programming paradigm based on the idea of asynchronous even processing and data streams.

For more details on what Reactive Programming is, check out: https://www.baeldung.com/cs/reactive-programming

N.B: Knowing the details of how reactive programming works under the hood is not absolutely necessary to understand Yew or even use it at all.

### Yew Features

- Features a macro for declaring interactive HTML with Rust expressions. Developers who have experience using JSX in React should feel quite at home when using Yew.

- Achieves high level performance by minimizing DOM API calls for each render and by making it easy to offload processing to background web workers.

- Supports JavaScript interoperability, allowing developers to leverage NPM packages and integrate with existing JavaScript applications.

### How Yew works under the hood (high-level summary of how your Yew source code written in Rust becomes understood by your browser)

Yew source code is written in Rust and compiled to WebAssembly (WASM) using the wasm-unknown-unknown target. The resulting WASM binary and any necessary JavaScript and asset files are then packaged together using a build tool like Trunk to create a working UI that can be displayed in a browser.

#### Prepare to write your first Yew application

Asides having Rust installing Rust on your machine, you need to install two development tools. They are:

- WebAssembly target (wasm32-unknown-unknown)

- Trunk. It will be your build tool build your Yew artifacts into Wasm. There are three alternatives to Trunk. They are 'wasm-pack', 'wasm-run', and 'xtask-wasm' which is still in early development.

You can find installation instructions for these tools here: https://yew.rs/docs/getting-started/introduction

#### Meet Yew!

For a quick taste of Yew's syntax, here is an hello-world Yew app that simply renders "Hello World" on your browser's view pane:

```rust
// Hello world Yew Application with single function component 'App'

use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

Project Requirements:

To run the above code, you need to

1. Create a new cargo bin project called `hello_world_yew'
2. Add the 'yew' as a dependency crate by running command `$ cargo add yew -F csr`. Section 'Yew Dependencies' sheds more light on what the '-F csr' part (argument) of this command means.
3. Inside your 'main.rs' file, copy, paste, and save the above code
4. Inside your project's root-level dir, create a template `index.html` file. Its path should look similar to this; <pre>hello_world_yew/index.html'</pre>. Trunk needs it to build your yew project. Optionally change your title tag's value to 'Hello World Yew!'.
5. Run the project by using trunk to serve it via command `$ trunk serve -o`
6. Check 'http://localhost:8080' on your choice browser to see the project running.

N.B: Port 8080 is Yew's default port for running new projects.

#### Some Commonly Used Yew Dependencies

- yew: Depending on the nature of your project, you have to between features 'csr' (client-side render), 'ssr' (server-side render), or even 'ssr-hydrate'. This intro's is Client-Side Render.
- web-sys: For accessing DOM-specific constructs such as 'HtmlInputElement'
- serde-json: A JSON serialization file format crate. More specifically, you can parse marshalled/serialized/encoded/JSON received from a user or even an API endpoint.
- serde: A framework for serializing and deserializng Rust structures efficiently and generically.

#### Yew Fundamentals

Many times during development of your Yew applications, you would find yourself repeating some four actions. Knowing the following four concepts/actions is of paramount importance:

- Components; functional components (recommended) or Struct components (archiac, not recommended).

- `Properties`

- `Callback`

- Hooks (e.g. `use_state()`, `use_context()`, `use_effect`, e.t.c, or even writing your own custom hooks. You'll see examples of these further down below.

- Styles (inline CSS, external CSS, or CSS tooling/prepocessor/frameworks/, e.g TailwindCSS). Again, you would see an example of doing this futher down in this introductory text.

- Fetch API (for making HTTP requests to your backend - could be axum, actix-web, Rocket, C# .Net, Go's Gin or Fiber or Gorilla, Java Spring, NodeJS, Deno, Django, Flask, Ruby on Rails, PHP Laravel. It does not matter). I'd encourage you use a Rust-based framwork though ;)

##### Components

Components are the building blocks of Yew applications. It allows you to encapsulate logic and/or design-presentation within a UI element and then expose it other components in a non-circular hierarchical tree-form. Components have parent-child relationships between them. They are reusable.

As mentioned there are two flavors;

- Function component which you have just learned
- Struct component, not covered here. To see Struct component examples, see https://yew.rs/docs/advanced-topics/struct-components/lifecycle

Here is another example of a functional component. This example lends the opportunity to showcase how conditional rendering can be done in Yew:

```rust
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let some_value = "sample";
    let option_value = Some("an option");

    html! {
      <>

        if some_value == "sample" { //* Here, you can see conditional rendering in action
          <h1 class={some_value}>{"Hello World! I matched 'sample'."}</h1>
        } else {
          <h1 class={"invalid"}>{"Hello world! I matched 'an option'."}</h1>
        }

        if let Some(val) = option_value {
          <p>{"And variable 'option_value' has value: "}{val}</p>
        } else {
          <p>{"show no option"}</p>
        }

      </>
    }
}
```

##### Properties

Properties are useful for passing data down from a parent component to a child component. You derive Properties for a struct you wish to hold data that you want to pass around to sub-components.
Your target data type you wish to become a Property must also derive `PartialEq`.

Here is a code example:

```rust
/*
  Assume the name of the yew project is 'sample_yew_project'
 */

// * INSIDE a sample module called child_component.rs
// * The component inside this module is called ChildComponent.
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Prop {
    pub prop_one: String,
}

// * Your child-component must have a reference of the target Property struct (in this case, Prop)
// * ...as its parameter.
#[function_component(ChildComponent)]
pub fn child_component(prop: &Prop) -> Html {
    html! {
      <div>
        <p>{"I am a Child component"}
        </p>
        <p>{&prop.prop_one}{" received from a component that nested me"}
        </p>
      </div>
    }
}

// *  INSIDE a sample 'lib.rs' module
// *  The component inside this module is called ParentComponent
use yew::prelude::*;

mod child_component;

use child_component::ChildComponent;

#[function_component(App)]
pub fn app() -> Html {
    html! {
      <>
        <div>
          <ChildComponent prop_one="prop_one value" />
        </div>
      </>
    }
}

// * INSIDE required module main.rs
// * Here you must have code to render the super parent component of your yew app
// * ...which in this case is the component inside module lib.rs
use sample_yew_project::App;

fn main() {
  yew::Renderer::<App>::new().render();
}
```

For greater detail about `Properties`, refer to this piece of Yew.rs documentation - https://yew.rs/docs/concepts/function-components/properties

##### Callback

A Callback, not to confused with a callback/callback-function which is simply functions you pass as values for events such as on_click, is particularly useful for passing data back up to a parent component from a child component. You couldn't just can do this with `Properties`.

Here is an example:

```rust

```

##### Hooks

Hooks are utility functions you call inside components streamline logic inside. Hooks are wrapper arounds common-reoccuring patterns, logic, or behavior, for example there is a hook called 'use_state()' which allows you to rerender a component each time the value passed to it changes. Yew itself provides you with some hooks (e.g, 'use_state()'). Of course, you can create your own custom hook.

Here, you'll learn about the following hooks:

- `use_state()`

- `use_context()`

- `use_effect()` or `use_effect_with_deps`

###### use_state()

This is perhaps the most important hook in Yew. It allows you to specify that should a particular value in your the component enclosing the state change, then that component must be re-rendered to reflect such change.

Example:

```rust

```

###### use_context()

!Todo: Explain in plain words first.

Example:

```rust

```

###### use_effect(), use_effect_with_deps

Both hooks allows you perform actions / side effects immediately after its enclosing component has finished rendering first time, or every time any one of a tuple of components change. 'use_effect_with_deps' is a little bit special because unlike 'use_effect()', it allows you to call/utilize depencies right inside it the hook's callback. Both hooks allow you to specify an optional destructor. You can check this documentation for nitty-gritty details - https://docs.rs/yew/latest/yew/functional/fn.use_effect.html

```rust
{
    let state_data = state_data.clone();

    // fetch data and set it as 'state_data' new value
    use effect_with_deps( move |_| {
        let state_data = state_data.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let fetched_data: FetchedDataType = Request::get("https://fetch-data-axum-endpoint/just-a-path.anything")
                .send()
                .await
                .unwrap()
                .json() // asumming FetchedDataType is actually Json
                .await
                .unwrap();
            state_data.set(fetched_data);
        });

        || ()

    }, ()); // end of 'use_effect_with_deps'
}
```

##### Applying Styles

You can appy styles to your Yew app in three ways:

- inline

- external stylesheet

- CSS preprocessors/toolings like TailwindCSS.

Examples of each one are shown below.

###### Inline styles

!To-Do: Show example code for completeness reasons

###### External styles

!To-Do: Show example code for completeness reasons

###### TailwindCSS

!To-Do: Show example code for completeness reasons

##### Fetch API

You can make HTTP request to any backend service from inside your Yew application using asynchronous `gloo_net::http::Request` and `wasm_bindgen_futures' (which bridges Rust's async with JavaScript Promise).

Here is a sample code snippet that fetches data from an axum API, and then update a state with the returned data, hence causing a re-render of the screen to display the changes:

```rust
use gloo_net::http::Request;

#[function_component(SampleComponent)]
fn a_sample_component() -> Html {


    {
        use_effect_with_deps(move |_| {

            wasm_bindgen_futures::spawn_local(async move {
                let fetched_data: Json(SomeData) = Request::get("https://any-backend/data.json")
                    .send() // i.e. Send request
                    .await
                    .unwrap()
                    .json()
                    .await

            }, ()); // end of invoked 'use_effect_with_deps'
        })
    }
}
```

<b>N.B</b>:

- When your Yew app fetches data from a given API, then provided your Yew project uses Trunk as its build tool, your trunk command to run your app must look like this:

```
$ trunk serve --proxy-backend=https://your-api-endpoint.com
```

- On your development machine while using localhost for your backend, your backend-API port must be running on localhost, and you must refer to localhost:8080 as the target port too, as in;

```
$ trunk serve --proxy-backend=http://localhost:8080
```

#### Testing

Yew's official test patterns are still in development. Keep tabs on this page so you can learn them later https://yew.rs/docs/more/testing

#### Useful / Helful third-party libraries to use in your Yew web-app

- wasm-bindgen: a library and tool to facilitate high-level interactions between Wasm modules and JavaScript; it is built with Rust by The Rust and WebAssembly Working Group.

- wasm-bindgen-futures: Depends on `wasm-bindgen`. This crate bridges the gap between a Rust `Future` and a JavaScript `Promise`. Its `spawn_local` interface is particularly useful for fetching resource from a backend.

- js-sys: Depends on `wasm-bindgen`. This crate allows you to create bindings to JavaScript's standard, built-in ojects, including their methods and properties. This does not include any Web, Node, or any other JS environment APIs. Only the things that are guaranteed to exist in the global scope by the ECMAScript standard. Check https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects to see the built-in ojects `js_sys` create bindings to.

- web-sys: Depends on `wasm-bindgen`. It provides raw API bindings for Web APIs imported inside `wasm-bindgen`. Each type in `web-sys` has its on cargo feature. With web-sys, you can access `window.fetch`, `Node.prototype.appendChild`, WebGL, WebAudio, and many more!. Find out here: https://rustwasm.github.io/wasm-bindgen/web-sys/index.html. To access ECMAScript standards-compliant JavaScript global objects such as Array, Date, and eval, you'd need to use `js-sys` crate.

- gloo-net: Depends on `web-sys`. It is an HTTP requests library for WASM Apps. It provides idiomatic Rust bindings for `web-sys`' `fetch`, `WebSocket`, and `EventSource` APIs. For more details, check: https://crates.io/crates/gloo-net

Keep tabs on this page to know about more third-party libraries:

https://yew.rs/community/external-libs

#### More on Yew

What does Yew's support for gRPC and GraphQL APIs look like?

To quote one of Yew's maintainers on Discord (username: intendednull);

"
Support is pretty much the same for any web framework. In that it works with any grpc or graphql lib that is compatible with wasm. Should be anything that is written in pure rust
"

Notable Yew concepts not convered here:

- Router: https://yew.rs/docs/concepts/router

- Agents: https://yew.rs/docs/concepts/agents

- Suspense: https://yew.rs/docs/concepts/suspense

#### Sample Yew app that sends and HTTP request to an axum API end-point

Please refer to the binary project called `yew_backend` accompanying this repo.

#### Yew ecosystem

Discord: https://discord.gg/yew-701068342760570933

<b>
Thank you for reading this introductory material on Yew!.
</b>
