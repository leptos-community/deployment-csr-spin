<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Deploy Leptos CSR Apps on Spin

After installing both the Spin framework and Leptos, then clone this repo and run `trunk watch` in one terminal while also running `spin watch` in another terminal -- when making changes in your Leptos app, the only thing you need to do is reload the browser window: Leptos and Spin will take care of the rest!

For building and releasing the app, run `trunk build --release` and `spin deploy` to get your app up and running in the cloud!

---

If you'd like to deploy an existing Leptos CSR app to Spin:

1) Ensure you have the Spin CLI installed
2) in your CLI, move to the directory that contains your existing Leptos CSR app - for example purposes, we'll call this project directory "leptos-csr"
3) set up a new Spin project by using `spin new`
4) in the `spin new` CLI prompt, select the "static-fileserver" option
5) When the `spin new` CLI prompts you to provide a name for your application, provide the directory name of your existing project, eg. `leptos-csr`
6) Confirm that you want to generate the Spin project even though your direcotry already contains other files
7) Add a description for your project
8) for the HTTP path, use `/` instead of the default (/static/...)
9) for the directory containing the files to serve, enter `dist` (which is where the Leptos CSR app will automatically output your files when you run `trunk watch` or `trunk build --release`)
10) enter your directory and start developing with `trunk watch` & `spin watch` in separate terminals
11) to build and deploy your app, run `trunk build --release`, then  `spin deploy`
12) Profit!

Optionally, you can add a build command to your `spin.toml` manifest file to automatically build Leptos for deployment just using the `spin build` & `spin deploy` commands. For example, if your project is named "leptos-csr", then include the following somewhere in your `spin.toml` file:

```
[component.leptos-csr.build]
command = "trunk build --release"
```

Further info about developing with Spin can be found in their excellent documentation.
