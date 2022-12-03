# SQL to mermaid.js Entity Relation Diagram
## This web app helps visualize SQL table creation scripts.

## How to use

1. Past your sql statements
2. Click the refresh button
3. Visualize tables and relationships in a entity relation diagram
4. Copy the generated mermaid.js script to embed the schema in your markdown technical documentation

## Implementation

- sql parsing using rust
- front end web app using yew framework
- deployment on github pages

## Run locally
### How to install

```sh
npm install
```

### How to run in debug mode

```sh
# Builds the project and opens it in a new browser tab. Auto-reloads when the project changes.
npm start
```

### How to build in release mode

```sh
# Builds the project and places it into the `dist` folder.
npm run build
```

### How to run unit tests

```sh
# Runs tests
npm test
```

### Deploy

```sh
git tag vα.β.γ
git push origin vα.β.γ
```