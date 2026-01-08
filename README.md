<div align="center">

  <a href="https://github.com/vector-sur/.github/blob/main/images/logo.svg"><img src="https://github.com/vector-sur/.github/blob/main/images/logo.svg" alt="VectorSur logo" width="200"></a>

# Vector Sur Backend

</div>

## Architecture Diagram

```mermaid
flowchart LR
    User((End User))
    AdminUser((Administrator))

    subgraph SystemBoundary ["System"]
        direction TB
        WebApp["Web App"]
        MobileApp["Mobile App"]
        AdminPanel["Admin Dashboard"]
        
        Backend["Backend API"]
        Database[("Database")]
    end

    User <--> WebApp
    User <--> MobileApp
    AdminUser <--> AdminPanel

    WebApp <-->|JSON/HTTPS| Backend
    MobileApp <-->|JSON/HTTPS| Backend
    AdminPanel <-->|JSON/HTTPS| Backend

    Backend <--> Database
```

## Class Diagram

```mermaid
classDiagram

    Order *-- User
    Trip *--  Order
    Trip *-- Location : from
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
    Business --o Location

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

        +active_accounts: int
        +inactive_accounts: int
        +total_accounts: int
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
        +active: boolean
    }

    class Business {
        +name: string
        +description: string
        +owner_id: int
        +location_id: int | null
        +verified: boolean
        +active: boolean
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
      +active: boolean
    }

    class Admin {
      +user_id: int
    }

    class Drone {
        +name: string
        +number: int
        +user_id: int
        +active: boolean
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
    [*] --> Requested : User.request_trip()
    Requested --> Unfinished : Drone.system_error()
    Delivered --> Finished: Drone.finish_trip()
    Requested --> Canceled: User.cancel_trip()
    Delivered --> Unfinished: Drone.system_error()
    Unfinished --> [*]
    Canceled --> [*]
    Requested --> Delivered: Drone.start_return_trip()
    Finished --> [*]
```

## Authentication

The backend uses **JWT** for authentication with **bcrypt** for password hashing.

### Password Security

- **Hashing Algorithm**: bcrypt with salt (cost factor: 12)
- **Salt**: Automatically generated and embedded in the hash by bcrypt
- **Hash Format**: `$2b$12$[22-char-salt][31-char-hash]`

### JWT Configuration

- **Algorithm**: HS256 (HMAC with SHA-256)
- **Token Expiration**: 1 hour
