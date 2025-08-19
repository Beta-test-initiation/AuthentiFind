# Project: AuthentiFind

## 1. Project Vision

AuthentiFind is a high-performance service built in Rust that scrapes secondhand luxury marketplaces to discover high-value fashion items. It then cross-references these items with a digital ledger to verify their authenticity and track their provenance, providing buyers with confidence and a rich history for each piece.

---

## 2. Core Technologies

* **Language:** Rust
* **Asynchronous Runtime:** `tokio`
* **Networking:** `reqwest` for HTTP requests
* **Data Parsing:** `scraper` for HTML parsing
* **Database:** `sqlx` with PostgreSQL or SQLite
* **Cryptography:** `sha2` or similar for generating unique identifiers
* **Serialization:** `serde` for handling data (JSON, etc.)
* **Web Framework (Optional):** `Axum` or `Actix-web`

---

## 3. Development Phases

### Phase 1: The Scraper Engine

* **Objective:** Build a robust, asynchronous web scraper capable of extracting specific item data from target websites.
* **Key Tasks:**
    1.  Initialize a Rust project with `tokio` as the async runtime.
    2.  Implement a scraping module that uses `reqwest` to fetch the HTML content of a search results page.
    3.  Use the `scraper` crate to parse the HTML, targeting specific CSS selectors to extract item details: name, price, URL, and any listed serial numbers or unique identifiers.
    4.  Structure the output into a clean, well-defined Rust struct.
* **Success Criteria:** The application can be run from the command line with a search term and successfully prints the structured data of found items to the console.

---

### Phase 2: The Digital Ledger Prototype

* **Objective:** Create a standalone system for item registration and verification that acts as a source of truth.
* **Key Tasks:**
    1.  Define a database schema for storing items and their history using `SQLx`.
    2.  Create a `ledger` module with core functions:
        * `register_item(details)`: Accepts item details, generates a unique SHA-256 hash from key properties (e.g., `brand` + `serial_number`), and saves the new item "token" to the database.
        * `check_provenance(identifier)`: Queries the database for an item using its serial number or other unique ID and returns its known history.
* **Success Criteria:** The ledger module can successfully register a new item, assign it a unique hash, and retrieve its history programmatically.

---

### Phase 3: System Integration

* **Objective:** Combine the Scraper Engine and the Digital Ledger into a single, cohesive application.
* **Key Tasks:**
    1.  Refactor the project to include both the `scraper` and `ledger` modules.
    2.  Modify the scraper's main loop: after an item's data is successfully extracted, its serial number is passed to the `ledger::check_provenance()` function.
    3.  Augment the final output. The application should now present the scraped listing data alongside any historical information found in the ledger (e.g., "Provenance Found: Yes" or "Provenance: Not Found").
    4.  Implement a basic notification system, such as a formatted printout to the console or a webhook message to a Discord channel.
* **Success Criteria:** The end-to-end system can find an item online, automatically query the internal ledger for its history, and present a unified report to the user.

---

### Phase 4: API & Expansion (Stretch Goals)

* **Objective:** Expose the core functionality through a web API and add user-facing features.
* **Key Tasks:**
    1.  Wrap the integrated logic in a web server using `Axum` or `Actix-web`.
    2.  Create API endpoints:
        * `POST /api/searches`: To start a new scraping job.
        * `GET /api/items/{id}`: To view details and provenance of a found item.
    3.  Implement a user account system to allow users to manage their "digital collection" of items they've purchased and registered.
* **Success Criteria:** The system is fully operational as a web service, capable of being controlled and queried via HTTP requests.