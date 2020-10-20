# wpfxm

This is a desktop application to make managing and launching Wine applications easier and less tedious.

It is very much a WIP, and currently has the following features:

- Automatic prefix scanning
- Automatic application scanning
- Application launching

This project is made with [Electron](https://www.electronjs.org/), [Preact](https://preactjs.com/), [NodeJS](https://nodejs.org/), and [TypeScript](https://www.typescriptlang.org/).

# Building

This project requires the following dependencies:

- npm
- yarn (optional, but preferred)

If you do not use yarn to build the project, you may run into build problems as dependency versions will not be locked. To install yarn, run `npm install yarn` in the project directory. On Linux, you should also be able to install it through your distribution's package manager.

The following steps will install the rest of the dependencies and build the project:

## Yarn

1. `yarn install`
2. `yarn build`

## npm

1. `npm install`
2. `npm run build`

There is not currently a build script to package the application up into a single executable, so for now you will need to run either `yarn electron` or `npm run electron` in the project directory to launch it.
