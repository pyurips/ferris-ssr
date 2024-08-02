
![Captura de tela 2024-08-01 042726](https://github.com/user-attachments/assets/7b37309e-2c6e-45e8-be2c-b009cbf0f233)
**Ferris SSR** is a template for Server-Side Rendering (SSR) that features a system for componentizing HTML elements, passing any type of data to components, and managing application state.

#### Security considerations
Note that this template does not include security mechanisms for common threats such as XSS (Cross-Site Scripting). Developers should be aware of these potential vulnerabilities and implement appropriate security measures as needed.

#### Technologies
- **Actix**: A fast and powerful web framework for Rust, used to build high-performance APIs and web applications.
- **Alpine.js**: A lightweight JavaScript framework for creating dynamic interfaces using a declarative approach similar to Vue.js.
- **HTMX**: A JavaScript library that enables adding interactivity to traditional HTML applications, making it easy to load dynamic content without the need for full front-end frameworks.
- **Tailwind CSS**: A utility-first CSS framework that allows you to quickly build custom user interfaces using pre-defined, highly configurable utility classes.

#### Concepts
**Fragments** are strings relevant for assembling HTML. They can be HTML components to be rendered or any other type of data that, in the end, will be converted into a string to be rendered to the user. **Components** are snippets of HTML that will be converted into strings, manipulated, and sent to users making the request. When obtaining a component using the `spawn` method, it is possible to pass a hash map in the arguments with the name of the fragment and the value to be substituted in the final string. For example:
```rust
increase_button_data.insert("color".to_string(), "bg-stone-100".to_string());
increase_button_data.insert("text".to_string(), "Increase the counter".to_string());

let increase_particle_button = component.spawn("particles/increase_button", Some(&increase_button_data));
```
In this example, I am not adding another component but rather a generic fragment, which is the data that will be rendered. In the HTML that will be rendered, there should be two placeholders in the file: `__color__` and `__text__`, which will be replaced correctly. However, another type of fragment, considered a component, can also be added:
```rust
let increase_particle_button = component.spawn("particles/increase_button", Some(&increase_button_data));
let htmx_particle_button = component.spawn("particles/htmx_button", None);

let mut components = HashMap::new();
components.insert("button".to_string(), increase_particle_button.unwrap());
components.insert("htmx_button".to_string(), htmx_particle_button.unwrap());

let main_component = component.spawn("main", Some(&components));
```
Therefore, to render the components correctly, the HTML file must contain `__button__` and `__htmx_button__`.

#### About performance
To perform load performance testing, [Artillery.](https://www.artillery.io/) was used. The test configuration file, `load_test.yml`, is available in this repository. I conducted a performance comparison between Next.js (Node), Fresh (Deno), and Ferris SSR. The default landing page generated when the projects are first created was used. The results are as follows:

Next.js (Node) | Apdex: 0.54
![next_test](https://github.com/user-attachments/assets/a7d2d980-5ac5-4447-b711-4bd51c8c0153)

Fresh (Deno) | Apdex: 0.64
![fresh_test](https://github.com/user-attachments/assets/a075d680-4820-4191-839f-c3e31131354d)

Ferris SSR (Rust) | Apdex: 1
![rust_test](https://github.com/user-attachments/assets/747ad780-08ac-425c-9d20-7d78531ac28f)

#### How to run
```
git clone git@github.com:pyurips/ferris-ssr.git
```

Enter the directory where the clone was made.

```
cargo build --release
```

```
cargo run --release
```

