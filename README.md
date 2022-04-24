# Quasar

This is a project using Rust backend and Svelte frontend 
-
#### Backend Stack
* For backend side we're using the <strong>Rust</strong> programmig language, 
<strong>Actix-Web</strong> web framework, <strong>Sea-Orm</strong> and <strong>Postgres</strong> Database.
---

#### Launching Backend 
* Pre-Requisites:
	* Rust Programming language (Rustup toolchain, Cargo package manager)
	* Sea-Orm CLI
	* Postgres Database
	* Sqlx CLI (optional)
	
* For launching the application cd into rust_backend directory and make a .env file at the root of the project, and add database_url, host, port and client_url variables in .env files and assign    	appropriate values to variables.
*  $ <strong> cd rust_backend </strong>

   after that
 * From the rust_backend/ directory run the following command:
	 $ <strong>cargo run</strong>
* The application will be launched on localhost:port, where port is port variable defined in .env file.
---
#### Frontend Stack
* For frontend we'll be using <strong>Typescript</strong> programming language, <strong>Svelte</strong> web framework, for styling we'll use <strong>Tailwind</strong> css framework and <strong> Alpinejs </strong> for adding javascript behaviour.
---

#### Launching Frontend
* Pre-Requisites
	* Node
	* yarn or npm
	* Typescript (optional for global install, IDE with Typescript support like VS Code recommended).

* To get started, navigate to svelte_frontend/ directory
* $ <strong> cd svelte_frontend </strong>

	after that
* From the svelte_frontend/ directory run the following command:
	* $ <strong>npm install</strong>	or <strong>yarn install</strong>,  this will download all required packages in package.json
* Now, run the following command to start the application
	* $ <strong>yarn dev</strong> or <br> <strong>$ npm run dev</strong>


