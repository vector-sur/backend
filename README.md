# Backend

## Class Diagram

```mermaid
classDiagram

    Order *-- User
    Trip *--  Order
    Trip *-- Location : from
    Trip *-- Location : to
    Trip *-- Drone
    Trip --o Report
    Report *-- ReportCategory
    ProhibitedZone *-- Location
    User --o Business : admin
    User *--o Business : employee
    User --o Drone : owns
    Product *-- Business
    Order *--* OrderDetail
    OrderDetail *--* Product
    User -- Person

    class Person {
        +name: string
        +lastname: string
        +phone: int
        +email: string | null
    }

    class OrderDetail {
        +amount: int
    }

    class Stats {
        +total_trips: int
        +today_trips: int
        +weekend_trips: int
        +monthly_trips: int

        +avg_delivery_time: float
        +avg_packing_time: float

        +avg_battery_consumption_per_km: float

        +cancellation_rate: float

        +accounts: int
    }

    class Order {
        +number: int
        +date: date_time
        +state: OrderState
        +price: float
    }
    
    class Trip {
        +request_time: date_time
        +weight: float
        +distance: float
        +est_time: float
        +state: TravelState
        +packing_time: float | null
        +battery_init: float | null
        +arrival_time: float | null
        +delivery_time: float | null
        +real_distance: float | null
        +battery_arrival: float | null
        +round_trip_time: float | null
        +total_time: float | null
        +battery_end: float | null
        +canceled_time: date_time | null
        +unfinished_time: date_time | null
        
        +new()
    }

    class Location {
        +name: string | null
        +latitude: float 
        +longitude: float 
    }

    class Product {
        +name: string
        +description: string 
        +price: float 
    }

    class Business {
        +name: string
        +description: string
        +amount_trips: int
        +amount_unique_trips: int
    }

    class ProhibitedZone {
        +description: string
        +since: date_time
        +to: date_time | null
        +radius: float
    }

    class UserStats {
        +trips: int
        +canceled_trip: int
        +last_login: int
    }

    class User {
      +username: string
      +password_hash: string
      +salt: string
    }

    class Drone {
        +name: string
        +number: int
        +user_id: int
    }

    class Report {
        +timestamp: date_time
        +title: string
        +description: string
    }

    class ReportCategory {
        +name: string
    }

    class TripState {
        <<enumeration>>
        +Requested
        +Delivered
        +Finished
        +Canceled
        +Unfinished
    }

    class OrderState {
        <<enumeration>>
        +Requested
        +Finished
        +Canceled
        +Uncomplete
    }
```
## Trip State Diagram

```mermaid
stateDiagram-v2
    [*] --> Requested : User.request()
    Requested --> Unfinished : Drone.system_error()
    Delivered --> Finished: Drone.finish_trip()
    Requested --> Canceled: User.cancel_trip()
    Delivered --> Unfinished: Drone.system_error()
    Unfinished --> [*]
    Canceled --> [*]
    Requested --> Delivered: Drone.start_return_trip()
    Finished --> [*]
```

```
src/
├── main.rs                 
├── lib.rs                  
├── config/
│   ├── mod.rs             
│   └── database.rs        
├── models/
│   ├── mod.rs
│   ├── drone.rs           
│   ├── user.rs
│   └── trip.rs
├── handlers/              
│   ├── mod.rs
│   ├── drone.rs           
│   ├── user.rs
│   └── trip.rs
├── routes/                
│   ├── mod.rs
│   ├── drone.rs           
│   ├── user.rs
│   └── trip.rs
├── services/              
│   ├── mod.rs
│   ├── drone_service.rs
│   └── trip_service.rs
├── middleware/
│   ├── mod.rs
│   ├── auth.rs           
│   └── logging.rs
└── utils/
    ├── mod.rs
    └── error.rs         
```

## Authentication

The backend uses **JWT (JSON Web Tokens)** for authentication with **bcrypt** for password hashing.

### Password Security

- **Hashing Algorithm**: bcrypt with salt (cost factor: 12)
- **Salt**: Automatically generated and embedded in the hash by bcrypt
- **Hash Format**: `$2b$12$[22-char-salt][31-char-hash]`

### JWT Configuration

- **Algorithm**: HS256 (HMAC with SHA-256)
- **Token Expiration**: 1 hour

### API Endpoints

#### Register
```bash
POST /auth/register
Content-Type: application/json

{
  "username": "johndoe",
  "name": "John",
  "lastname": "Doe",
  "phone": 1234567890,
  "email": "john@example.com",
  "password": "your-password"
}
```

Response:
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user_id": 1,
  "username": "johndoe"
}
```

#### Login
```bash
POST /auth/login
Content-Type: application/json

{
  "username": "johndoe",
  "password": "your-password"
}
```

Response:
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user_id": 1,
  "username": "johndoe"
}
```

#### Protected Endpoint Example
```bash
GET /protected
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

Response:
```json
{
  "message": "You are authenticated!",
  "user_id": "1",
  "username": "johndoe"
}
```

### Using Authentication in Routes

To protect a route, simply add `Claims` as a parameter:

```rust
async fn protected_route(claims: Claims) -> Json<Response> {
    // claims.sub contains the user_id
    // claims.username contains the username
    Json(Response {
        user_id: claims.sub,
        username: claims.username,
    })
}
```