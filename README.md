# Flight REST API

A RESTful API for managing flight information and bookings.

## Overview

This API provides endpoints for searching flights, managing bookings, and retrieving flight information. Built with modern best practices for scalability and maintainability.

## Features

- Search for available flights
- Book and manage flight reservations
- Retrieve flight details and schedules
- User authentication and authorization
- Real-time flight status updates

## Getting Started

### Prerequisites

- Node.js (v14 or higher) / Python (v3.8 or higher) / Java (v11 or higher) - *adjust based on your stack*
- Database (PostgreSQL/MySQL/MongoDB) - *adjust based on your choice*
- API keys for external services (if applicable)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/pouyan6/flight-api.git
   cd flight-api
   ```

2. Install dependencies:
   ```bash
   npm install
   # or
   pip install -r requirements.txt
   ```

3. Set up environment variables:
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

4. Run database migrations:
   ```bash
   npm run migrate
   ```

5. Start the server:
   ```bash
   npm start
   ```

## API Endpoints

### Flights

- `GET /api/flights` - Get all flights
- `GET /api/flights/:id` - Get flight by ID
- `POST /api/flights` - Create a new flight (admin)
- `PUT /api/flights/:id` - Update flight (admin)
- `DELETE /api/flights/:id` - Delete flight (admin)

### Bookings

- `GET /api/bookings` - Get user's bookings
- `POST /api/bookings` - Create a new booking
- `GET /api/bookings/:id` - Get booking details
- `PUT /api/bookings/:id` - Update booking
- `DELETE /api/bookings/:id` - Cancel booking

### Authentication

- `POST /api/auth/register` - Register new user
- `POST /api/auth/login` - Login user
- `POST /api/auth/logout` - Logout user

## Usage Example

```javascript
// Search for flights
fetch('https://api.example.com/api/flights?from=NYC&to=LAX&date=2026-02-01')
  .then(response => response.json())
  .then(data => console.log(data));
```

## Configuration

Key configuration options in `.env`:

```
PORT=3000
DATABASE_URL=your_database_url
JWT_SECRET=your_secret_key
API_KEY=your_api_key
```

## Testing

Run the test suite:

```bash
npm test
```

Run integration tests:

```bash
npm run test:integration
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

Project Link: [https://github.com/pouyan6/flight-api](https://github.com/pouyan6/flight-api)

## Acknowledgments

- Thanks to all contributors
- Inspired by modern REST API best practices
