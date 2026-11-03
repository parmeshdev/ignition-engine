# 5 Core Engines (The Node.js Translation)

## 1) tikio ( The Event Loop )
### - Node.js Equivalent: The V8 Engine/libuv
### - Purpose: Rust dose not have an async event loop built-in. Tokio is the multi-threaded engine that allows us to use async/await and handles thousands of concurrent requests across all your CPU cores.

## 2) axum (The Router):
### - Node.js Equivalent: Express.js or Fastify
### - Purpose: The HTTP web framework. we will enable the macros feature for the better error messages.

## 3) serde ( The Translator ):
### - Node.js Equivalent: JSON.parse() and JSON.stringify().
### - Purpose: It strictly translates incoming raw network bytes into Rust memory structs, and vice versa.

## 4) sqlx ( The Database Driver ):
### - Node.js Equivalent : pg ( node-postgres ) combines with Prisma.
### - Purpose: This is our async PostgreSQL driver. its superpower is that it checks your SQL queries at compile time. we enable the postgres and runtime-tokio-rustls ( for async TLS connections ) features.

## 5) dotenvy( The Environment Loader):
### - Node.js Equivalent : dotenv.
### - Purpose: Loads your .env file so we can securely pass the PostgreSQL connection string.