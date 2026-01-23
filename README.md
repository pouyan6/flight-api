# Flight REST API

A RESTful API for managing flight plans built with Rust, Actix-web, and SQLite. This API provides secure endpoints for creating, retrieving, and managing flight plan data with bearer token authentication.

## 🚀 Features

- **Flight Plan Management**: Create, retrieve, list, and delete flight plans
- **User Authentication**: API key-based bearer token authentication
- **SQLite Database**: Lightweight database for storing flight plans and users
- **CORS Support**: Configured for cross-origin requests
- **Logging**: Request logging with configurable levels
- **RESTful Design**: Clean REST API structure with JSON responses

## 📋 Prerequisites

- Rust (2024 edition)
- Cargo
- SQLite (bundled with rusqlite)

## 🛠️ Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/pouyan6/flight-api.git
   cd flight-api
   ```

2. **Build the project:**
   ```bash
   cargo build --release
   ```

3. **Run the server:**
   ```bash
   cargo run
   ```

   The server will start on `http://0.0.0.0:3000`

## 📚 API Endpoints

### Health Check

- `GET /` - Server health check

### User Management

- `POST /api/v1/admin/user` - Create a new user and generate API key
  ```json
  {
    "name": "John Doe",
    "api_key": ""
  }
  ```
  Response: Returns generated API key

### Flight Plans

All flight plan endpoints require Bearer token authentication.

- `GET /api/v1/flight_plans` - Get all flight plans
- `GET /api/v1/flight_plans/{flight_plan_id}` - Get flight plan by ID
- `POST /api/v1/flightplan` - Create a new flight plan
- `DELETE /api/v1/flight_plans/{flight_plan_id}` - Delete flight plan by ID

### Flight Plan Schema

```json
{
  "flight_plan_id": "string (UUID)",
  "altitude": 10000,
  "airspeed": 250,
  "aircraft_identification": "N12345",
  "aircraft_type": "C172",
  "arrival_airport": "KLAX",
  "departing_airport": "KJFK",
  "flight_type": "VFR",
  "departure_time": "2026-01-23T10:00:00Z",
  "estimated_arrival_time": "2026-01-23T16:00:00Z",
  "route": "Direct",
  "remarks": "None",
  "fuel_hours": 6,
  "fuel_minutes": 30,
  "number_onboard": 4
}
```

## 🔐 Authentication

The API uses Bearer token authentication. To authenticate requests:

1. Create a user via `/api/v1/admin/user` to receive an API key
2. Include the API key in the Authorization header:
   ```
   Authorization: Bearer YOUR_API_KEY
   ```

## 💻 Usage Example

### Create a User
```bash
curl -X POST http://localhost:3000/api/v1/admin/user \
  -H "Content-Type: application/json" \
  -d '{"name": "John Doe", "api_key": ""}'
```

### Create a Flight Plan
```bash
curl -X POST http://localhost:3000/api/v1/flightplan \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "flight_plan_id": "",
    "altitude": 10000,
    "airspeed": 250,
    "aircraft_identification": "N12345",
    "aircraft_type": "C172",
    "arrival_airport": "KLAX",
    "departing_airport": "KJFK",
    "flight_type": "VFR",
    "departure_time": "2026-01-23T10:00:00Z",
    "estimated_arrival_time": "2026-01-23T16:00:00Z",
    "route": "Direct",
    "remarks": "None",
    "fuel_hours": 6,
    "fuel_minutes": 30,
    "number_onboard": 4
  }'
```

### Get All Flight Plans
```bash
curl -X GET http://localhost:3000/api/v1/flight_plans \
  -H "Authorization: Bearer YOUR_API_KEY"
```

### Get Flight Plan by ID
```bash
curl -X GET http://localhost:3000/api/v1/flight_plans/{flight_plan_id} \
  -H "Authorization: Bearer YOUR_API_KEY"
```

### Delete Flight Plan
```bash
curl -X DELETE http://localhost:3000/api/v1/flight_plans/{flight_plan_id} \
  -H "Authorization: Bearer YOUR_API_KEY"
```

## 🏗️ Project Structure

```
flight-api/
├── src/
│   ├── main.rs           # Application entry point, server setup
│   ├── controller.rs     # API endpoint handlers
│   ├── database.rs       # Database operations and queries
│   └── schema.rs         # Data models (FlightPlan, User)
├── Cargo.toml            # Dependencies and project metadata
├── config.toml           # Configuration file
├── flights.sqlite        # SQLite database
└── README.md
```

## 🔧 Configuration

The server runs with the following configuration:
- **Host**: `0.0.0.0`
- **Port**: `3000`
- **Workers**: `2`
- **Log Level**: `info` (configurable via `RUST_LOG` env variable)

To change the log level:
```bash
RUST_LOG=debug cargo run
```

## 📦 Dependencies

Key dependencies used in this project:

- **actix-web** (4.3.0) - Web framework
- **actix-cors** (0.6.4) - CORS middleware
- **actix-web-httpauth** (0.8.0) - Authentication middleware
- **rusqlite** (0.29.0) - SQLite database interface
- **serde** (1.0.147) - Serialization/deserialization
- **uuid** (1.4.2) - UUID generation
- **env_logger** (0.10.0) - Logging

See `Cargo.toml` for complete dependency list.

## 🧪 Testing

```bash
# Run tests
cargo test

# Run with verbose output
cargo test -- --nocapture
```

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the Apache-2.0 License - see the [LICENSE](LICENSE) file for details.

## 👤 Author

**Pouyan V**

## 📞 Contact

Project Link: [https://github.com/pouyan6/flight-api](https://github.com/pouyan6/flight-api)

## 🙏 Acknowledgments

- Built with Actix-web framework
- Inspired by modern REST API best practices
- Thanks to the Rust community

---

**Note**: This API is designed for managing flight plan data. The flight_plan_id is automatically generated as a UUID when creating new flight plans.
