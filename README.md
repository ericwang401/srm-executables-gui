<h1><img src="https://github.com/ericwang401/srm-executables-gui/assets/37554696/15efa103-d05c-4808-9fe1-afb4837a2a8f" height="30"/> SRM Executables GUI</h1>

<img src="https://i.imgur.com/l0x3txH.png" width="500" />

A GUI for <a href="https://github.com/rgsadygov/SRM_executables">SRM Executables</a>, which is a command-line utility. Internally, this GUI is not a rewrite of SRM executables; it is a wrapper over the command-line interface, and a command is still executed.

## Contributors

-   Project led by <a href="mailto:Benjamin-Miller@omrf.org">Dr. Benjamin Miller</a> and <a href="mailto:Michael-Taylor@omrf.org">Dr. Michael Taylor</a> in the <a href="https://omrf.org/programs/aging-metabolism-research-program/">Oklahoma Medical Research Foundation Aging & Metabolism Research Program</a>
-   Programmed by <a href="https://github.com/ericwang401">Eric Wang</a>

## Downloads

Downloads are available in the releases section in the right side of your screen on desktop.

## Development

Once you've cloned this project and installed dependencies with `npm install` (or `pnpm install` or `yarn`), start a development server:

```bash
npm run tauri dev
```

## Production

To create a production version of SRM executables:

```bash
npm run tauri build
```
