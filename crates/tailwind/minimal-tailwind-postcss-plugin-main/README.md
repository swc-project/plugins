## Minimal Tailwind CSS integration reference

This is a simple PostCSS plugin designed to cover all of the tricky integration points needed for a tool like Tailwind to work.

The idea here is that if we can successfully port this plugin to an SWC CSS plugin, it will prove that everything we need to offer Tailwind as an SWC plugin exists.

This includes things like:

- Finding custom at-rules like `@tailwind` in the CSS and replacing them with our own generated CSS
- Reading a JavaScript configuration file and using information from that file to influence the generated CSS (like using a user's custom colors)
- Reading the user's configered template files to extract potential classes, and using that list to only generate the CSS that's needed
- Rebuilding the CSS file when the user changes their CSS files
- Rebuilding the CSS file when the user changes their JavaScript configuration file
- Rebuilding the CSS file when the user changes any of their template files

We've deliberately left out anything that is repetitive or low-risk as we don't need any help in those areas.

### Installation

```
npm install
```

### Tests

We've included tests for much of this behavior in `./index.test.js`, which you can run using the `test` script:

```
npm install
```

These tests cover all of the important transformations we need to be able to support, but exclude the behavior around when to trigger rebuilds as that is easier to capture with a demo project.

### Demo

There's a demo project in `./watching-demo` designed to test all of the scenarios that should trigger the CSS to be rebuilt.

You can run this demo by running the `demo` script from the project root:

```
npm run demo
```

This starts a small webpack project that is configured to use this PostCSS plugin. It will trigger a rebuild in the following situations:

- The input CSS file has changed.
- The `tailwind.config.js` file has changed.
- Any of the template files defined in the `content` section of the `tailwind.config.js` file have changed.
- Any time a new template file is added to a folder that is matched by a glob in the `content` section of the `tailwind.config.js` file.

We recommend opening `./watching-demo/dist/main.css` in a separate window so you can see that file being rebuilt any time you make any of the changes outlined above.
