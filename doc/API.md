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
        "length": 3,
        "items":
        [
          {
            "game_id": 1,
            "name": "The Legend of Zelda: Breath of the Wild",
            "original_price": 499,
            "current_price": 347.25,
            "lowest_price": 300,
            "link": "https://store.nintendo.com.hk/70010000009367",
            "image_url": "https://store.nintendo.com.hk/media/catalog/product/cache/6cfb139ec726e4601c9e927e52536377/1/1/110.jpg"
          },
          {
            "game_id": 2,
            "name": "Doraemon Story of Seasons",
            "original_price": 419,
            "current_price": 347.25,
            "lowest_price": 300,
            "link": "https://store.nintendo.com.hk/70010000019943",
            "image_url": "https://store.nintendo.com.hk/media/catalog/product/cache/08244de22d08060bc462ed2f13fdcc18/1/9/1920_1080__ch__1_.jpg"
          },
          {
            "game_id": 3,
            "name": "Fire Emblem 風花雪月",
            "original_price": 429,
            "current_price": 429,
            "lowest_price": 429,
            "link": "https://store.nintendo.com.hk/70010000021361",
            "image_url": "https://store.nintendo.com.hk/media/catalog/product/cache/08244de22d08060bc462ed2f13fdcc18/f/e/fe_1920_1080_ray_tc.jpg"
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
    - In JSON example on success, only the first and second have discount.

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
        "length": 1,
        "items":
        [
          {
            "game_id": 1,
            "name": "The Legend of Zelda: Breath of the Wild",
            "original_price": 499,
            "current_price": 347.25,
            "lowest_price": 300,
            "link": "https://store.nintendo.com.hk/70010000009367",
            "image_url": "https://store.nintendo.com.hk/media/catalog/product/cache/6cfb139ec726e4601c9e927e52536377/1/1/110.jpg"
          },
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
  
### Delete from wishlist
  - url
    - /api/del_wishlist
  - type
    - POST
  - attrs
    | name    | type      | required | desc.               | default | e.g.       |
    | ------- | --------- | -------- | ------------------- | ------- | ---------- |
    | user_id | number    | yes      | user's id           | -       | 533        |
    | game_id | number    | yes      | game's id           | -       | 1          |
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
        "message": "not exist"
      }
    }
    ```
  - msic


### Turn on/off mail alerts
### Delete from wishlist
  - url
    - /api/change_alert
  - type
    - POST
  - attrs
    | name    | type      | required | desc.               | default | e.g.       |
    | ------- | --------- | -------- | ------------------- | ------- | ---------- |
    | user_id | number    | yes      | user's id           | -       | 533        |
    | game_id | number    | yes      | game's id           | -       | 1          |
    | turn_on | boolean   | yes      | is turning on       | -       | false      |
    
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
        "message": "not exist in wishlist"
      }
    }
    ```
  - msic



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
  - url
    - /api/change_alert
  - type
    - POST
  - attrs
    | name    | type      | required | desc.               | default | e.g.       |
    | ------- | --------- | -------- | ------------------- | ------- | ---------- |
    | game_id | number    | yes      | game's id           | -       | 1          |
    
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
        "image_url": "https://store.nintendo.com.hk/media/catalog/product/cache/6cfb139ec726e4601c9e927e52536377/1/1/110.jpg",
        "desc": "以廣闊的世界為舞台，無論去哪裡、做什麼，冒險的一切由你決定。狩獵野生動物過活？去消滅魔物？逛盡各處景點？奔跑、游泳、滑翔、攀登，能在廣闊無垠的世界中享受自由自在的冒險。Nintendo Switch的話，可以在家裡的電視上仔細遊玩後繼續外出遊玩，就連遊戲方式都自由自在。
                【故事大綱】
                被稱為大災厄的災害發生了，海拉魯王國被滅亡了……
                100年後，主角林克從地下遺跡的長眠中甦醒，在不可思議的聲音引導下踏上大地。"
      }
    }
    ```
  - JSON example on fail
    ```json
    {
      "state":
      {
        "code": 1,
        "message": "game not exist"
      }
    }
    ```
  - msic

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
