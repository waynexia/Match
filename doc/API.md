# APIs of Match

Frontend part
------------------

## Index 
### Get game list
  - url
    - /api/game_list
  - type
    - GET
  - attrs
    | name | type | required | desc. | default | e.g. |
    | ---- | ---- | -------- | ----- | ------- | ---- |
    | -    | -    | -        | -     | -       | -    |
  - JSON example on success
    ```json
    {
      "state":
      {
        "code": 0,
        "message": "ok"
      },
      "data":
      {
        "items":
        [
          {
            "game_id": 1,
            "name": "The Legend of Zelda: Breath of the Wild",
            "discount": 0.75,
            "original_price": 499,
            "CNY_discounted": 329.71,
            "discount_area": "HK"
          },
          {
            "game_id": 2,
            "name": "Doraemon Story of Seasons",
            "discount": 0.9,
            "original_price": 6588,
            "CNY_discounted": 378.90,
            "discount_area": "JP"
          }
        ]
      }
    }
    ```
  - JSON example on fail
    ```json
    {
      "state":
      {
        "code": 1,
        "message": "server error"
      }
    }
    ```
  - msic
    - `original_price` is based on the `discount_area`

## Wish list
### Get wishlist
  - url
    - /api/get_wishlist
  - type
    - GET
  - attrs
    | name | type   | required | desc.     | default | e.g. |
    | ---- | ------ | -------- | --------- | ------- | ---- |
    | id   | number | yes      | user's id | -       | 533  |
  - JSON example on success
    ```json
    {
      "state":
      {
          "code": 0,
          "message": "ok"
      },
      "data":
      {
          "items":
          [
            {
              "game_id": 1,
              "name": "The Legend of Zelda: Breath of the Wild",
              "discount": 0.75,
              "original_price": 499,
              "CNY_discounted": 329.71,
              "discount_area": "HK"
            }
          ]
      }
    }
    ```
  - JSON example on fail
    ```json
    {
      "state":
      {
        "code": 1,
        "message": "user not exist"
      }
    }
    ```
  - msic

### Add wishlist
  - url
    - /api/add_wishlist
  - type
    - POST
  - attrs
    | name    | type      | required | desc.               | default | e.g.       |
    | ------- | --------- | -------- | ------------------- | ------- | ---------- |
    | user_id | number    | yes      | user's id           | -       | 533        |
    | game_id | number    | yes      | game's id           | -       | 1          |
    | TS      | timestamp | yes      | timestamp of adding | -       | 1561562361 |
  - JSON example on success
    ```json
    {
      "state":
      {
        "code": 0,
        "message": "ok"
      }
    }
    ```
  - JSON example on fail
    ```json
    {
      "state":
      {
        "code": 1,
        "message": "already exist"
      }
    }
    ```
  - msic
    - `original_price` is based on the `discount_area`
  
### Delete from wishlist

### Turn on/off mail alerts

## Feel lucky
### Get a random game
  - url
    - /api/random_game
  - type
    - GET
  - attrs
    No attribute
  - JSON example on success
    ```json
    {
      "state":
      {
          "code": 0,
          "message": "ok"
      },
      "data":
      {
            "game_id": 1,
            "name": "The Legend of Zelda: Breath of the Wild",
            "original_price": 499,
            "current_price": 347.25,
            "lowest_price": 300,
            "link": "https://store.nintendo.com.hk/70010000009367",
            "image_url": "https://store.nintendo.com.hk/media/catalog/product/cache/6cfb139ec726e4601c9e927e52536377/1/1/110.jpg"
      }
    }
    ```
  - JSON example on fail
    ```json
    {
      "state":
      {
          "code": 1,
          "message": "error"
      }      
    }
    ```

## Detail of game
### Get detail

## User 
### User login
  - url
    - /api/login
  - type
    - GET
  - attrs
    | name     | type   | required | desc.        | default | e.g.          |
    | -------- | ------ | -------- | ------------ | ------- | ------------- |
    | username | string | no       | user's name  | -       | Alison        |
    | email    | string | no       | user's email | -       | i@example.com |
    | password | string | yes      | password     | -       | 1#$!&         |
  - JSON example on success
    ```json
    {
      "state":
      {
          "code": 0,
          "message": "ok"
      },
      "data":
      {
          "user_id": 533,
          "username": "Alison",
      }
    }
    ```
  - JSON example on fail
    ```json
    {
      "state":
      {
          "code": 1,
          "message": "error"
      }      
    }
    ```
  - msic
    - `username` or `email` must have at least one.
    - `password` is encrypted.
  
### User sign up
  - url
    - /api/sign_up
  - type
    - POST
  - attrs
    | name     | type   | required | desc.           | default | e.g.          |
    | -------- | ------ | -------- | --------------- | ------- | ------------- |
    | username | string | yes      | user's name     | -       | Alison        |
    | email    | string | yes      | user's email    | -       | i@example.com |
    | password | string | yes      | user's password | -       | 1#$!&         |
  - JSON example on success
    ```json
    {
        "state":
        {
            "code": 0,
            "message": "ok"
        }
    }
    ```
  - JSON example on fail
    ```json
    {
      "state":
      {
          "code": 1,
          "message": "already exist"
      }
    }
    ```
  - msic

### Get new messages

Spider part
------------------

## add game
  - url
    - /api/spider/add_game
  - type
    - POST
  - attrs
    | name      | type   | required | desc.             | default | e.g.                                                                                                   |
    | --------- | ------ | -------- | ----------------- | ------- | ------------------------------------------------------------------------------------------------------ |
    | gamename  | string | yes      | game's name       | -       | The Legend of Zelda: Breath of the Wild                                                                |
    | price     | number | yes      | game's price      | -       | 499                                                                                                    |
    | link      | string | yes      | detail page       | -       | https://store.nintendo.com.hk/70010000009367                                                           |
    | image_url | string | yes      | title image's url | -       | https://store.nintendo.com.hk/media/catalog/product/cache/6cfb139ec726e4601c9e927e52536377/1/1/110.jpg |
    | desc      | string | yes      | description       | -       | 略                                                                                                     |
  - JSON example on success
    ```json
    {
      "state":
      {
        "code": 0,
        "message": "ok"
      }
    }
    ```
  - JSON example on fail
    ```json
    {
      "state":
      {
        "code": 1,
        "message": "server error"
      }
    }
    ```
  - msic

## new price
  - url
    - /api/spider/new_price
  - type
    - POST
  - attrs
    | name     | type      | required | desc.        | default | e.g.                                    |
    | -------- | --------- | -------- | ------------ | ------- | --------------------------------------- |
    | gamename | string    | yes      | game's name  | -       | The Legend of Zelda: Breath of the Wild |
    | price    | number    | yes      | game's price | -       | 374.24                                  |
    | date     | timestamp | yes      | timestamp    | -       | 1561562361                              |
  - JSON example on success
    ```json
    {
      "state":
      {
        "code": 0,
        "message": "ok"
      }
    }
    ```
  - JSON example on fail
    ```json
    {
      "state":
      {
        "code": 1,
        "message": "server error"
      }
    }
    ```
  - msic
