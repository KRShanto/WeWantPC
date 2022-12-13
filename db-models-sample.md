# Sample Database Models

Database models to be used in the application

This project is a e-commerce application which sells computer parts. The database models are as follows:

## User

- id
- name
- email
- password
- role
- createdAt

The role will one of the following

- Admin
- User
- Staff

If the role is staff, then it will also store the permissions of the user.

the syntax will be Staff{read, write, delete}

## Product

- id
- name
- name_variable
- description
- price
- quantity
- image_url
- createdAt
- inStock
- category_id
- brand
- rating

The `name_variable` will be the name of the product. It can also contain variables like the size, color, model, specification etc. This will fill the `name` field in the database.

The `name` will be the name of the product without any variables. This will be used for searching and filtering. This will be turned into a string after product creation and updated in the database.

The `description` will be the long description of the product. It will be used for the product page. This will be markdown text.

The `price` will be the price of the product. It will be in dollars.

The `image_url` will be the url of the image of the product. It will be a string. After creation, the image will be uploaded to the server and the url will be updated in the database.

The `category_id` will be the id of the category of the product. It will be a string.

## Category

- id
- name
- description

## Specification

- id
- names_values
- product_id

The `names_values` will be the specifications of the product. It will be a string. It will be in the format of `name:value,name:value,name:value`. The names and values will be separated by a comma. The name and value will be separated by a colon.

## Question

- id
- question
- user_id
- product_id
- createdAt

## Answer

- id
- answer
- user_id
- question_id
- createdAt
