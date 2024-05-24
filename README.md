# Pawn Pal API

Welcome to the Pawn Pal API! This small API returns a JSON list of all possible chess moves given a URL-encoded [FEN (Forsyth-Edwards Notation)](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation) string. It's perfect for integrating chess move generation into your projects.

## Try it yourself!

Go to your browser and type the following URL:

https://alpha.pawnpal.pbou.dev/standard/3r3r%2Fpkpn4%2F8%2F2p5%2F2P4p%2F2N1Q3%2FPP4P1%2F3R1R1K%20b%20-%20-%200%2026

## Features

- **Simple API**: Provides a straightforward interface to get possible chess moves from a given FEN string.
- **Standard Chess**: Currently, only the standard chess variant is supported. However, the underlying `shakmaty` library covers every Lichess variant, so additional variants can be added if needed.
- **Fast and Efficient**: Utilizes efficient open-source libraries to handle chess logic, serialization, and web serving.

## Open Source Dependencies

This project leverages several open-source projects, including:

- **[shakmaty](https://github.com/niklasf/shakmaty)**: Handles all the heavy lifting for chess move generation and FEN parsing.
- **[serde](https://github.com/serde-rs/serde)**: Manages serialization and deserialization of data to and from JSON format.
- **[actix-web](https://github.com/actix/actix-web)**: Powers the web server, handling HTTP requests and responses efficiently.

## Running the Project

### 1. Running with Rust

To run the project locally with Rust, follow these steps:

1. **Clone the repository**:

   ```sh
   git clone https://github.com/PierreBou91/pawn-pal
   cd pawn-pal
   ```

2. **Run the application**:

   ```sh
   cargo run
   ```

### 2. Running with Docker

To run the project using Docker:

1. **Build the Docker image**:

   ```sh
   docker build -t pawnpal .
   ```

2. **Run the Docker container**:

   ```sh
   docker run -p 8080:8080 pawnpal
   ```

### 3. Hosting on Cloud Providers

For cloud hosting, we strongly recommend [Railway](https://railway.app). Railway makes it extremely easy to deploy any app with minimal configuration.

## Testing the API

You can test the API using `curl` (or any web client). Here's an example:

```sh
curl http://localhost:8080/standard/6k1%2F4P3%2F5K2%2F8%2F8%2F8%2F8%2F8%20b%20-%20-%200%201
```

This will return a JSON array of possible moves for the given FEN string:

```json
[
  {
    "type": "Normal",
    "role": "King",
    "from": "G8",
    "capture": null,
    "to": "H7",
    "promotion": null
  },
  {
    "type": "Normal",
    "role": "King",
    "from": "G8",
    "capture": null,
    "to": "H8",
    "promotion": null
  }
]
```

Each move is a valid JSON object containing `type` (the type of move among `Normal`, `EnPassant`, `Castle` and `Put`) `role` (the piece that moved), `from` (starting position), `to` (ending position), `capture` (if any), and `promotion` (if applicable).

### Generating URL-encoded FEN

To generate a URL-encoded FEN string from any FEN string, you can use an online tool like [URL Encoder](https://www.urlencoder.org/).

## License

This project is licensed under the GPL 3.0 License due to the shakmaty project dependency licence. Feel free to use it in accordance with this license.

## Contributing

Contributions are welcome! Please fork the repository and submit pull requests for any enhancements or bug fixes.

## Contact

For any questions or support, please open an issue on the GitHub repository.

---

Thank you for using the Pawn Pal API! We hope it helps you with your chess-related projects. :D
