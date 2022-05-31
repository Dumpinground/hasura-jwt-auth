# Hasura JWT auth

Auth server that return JWT tokens to be used by Hasura.

## Setup

TODO: Add a whole docker-compose YAML as an example.

TODO: Publish to Docker Hub.

## Environment variables

### Required

* `JWT_SECRET` same string has to be provided to Hasura.
* `DATABASE_URL` url to the Postgres instance.

### Optional

* `HOST` change from the default host 0.0.0.0.
* `PORT` change from the default port 80.
* `JWT_ORG_CUSTOM_CLAIM` string to indicate which table and column to fetch an organisation ID from.
For example `user_metadata.org_id` fetches the value and adds it to the claim `x-hasura-organisation-id`.
Need a foreign key that points to the uuid columns `user.id`.
* `POST_REGISTER_URL` URL that will receive the email and id for newly created user.
* `POST_RESET_PASSWORD_URL` URL that will receive the email and ticket for the password that was reset.
* `POST_SET_PASSWORD_URL` URL that will receive the email for the user that set a new password.

## Hasura documentation to use JWT tokens

https://hasura.io/docs/latest/graphql/core/auth/authentication/jwt/

## License

MIT
