# **Email Newsletter Service**

An email newsletter delivery service built using [actix-web](https://github.com/actix/actix-web).

## **Project Goals**

The goal of this application is to learn how to build a production-level web application in Rust while exploring:

- Authentication strategies
- Fault-tolerant workflows
- Black-box testing
- Error handling

## **Features**

- ✅ Health check
- ✅ Session-based authentication
- ✅ Subscribe to newsletter
- ✅ Asynchronous queue-based email delivery
- ✅ Structured logging
- ✅ Idempotent requests
- ✅ Hierarchical configuration
- ⬜ Unsubscribe from newsletter _(Coming soon!)_

## **Technologies Used**

- **Database:** PostgreSQL
- **Session Store:** Redis
- **Web Framework:** Actix-web
- **Deployment:** Digital Ocean App Platform
- **Email Server:** Postmark

## **Installation**

### **Prerequisites**

- [Rust & Cargo](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/)
- [SQLx CLI](https://github.com/launchbadge/sqlx)

### **Setup Instructions**

```bash
# Clone the repository
git clone https://github.com/your-username/email-newsletter
cd email-newsletter

# Launch the database & session store containers
./scripts/init_db.sh
./scripts/init_redis.sh

# Run the application
cargo run
```

## **Testing**

The app has both unit tests and integration tests. You can run all tests with the following command:

```bash
cargo test
```
