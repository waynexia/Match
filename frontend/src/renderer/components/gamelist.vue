<template>
  <div id="gamelist">
    <game v-for="item in games.data.items" v-bind:data="item" @send="toDetail" v-bind:id="item.game_id"></game>
  </div>

</template>

<script>
  import game from './game'
  import axios from 'axios'
  import Vue from 'vue'
  import Self from './self'
  export default {
    name: 'gamelist',
    components:{
      game
    },
    mounted(){
      this.games = this.getGameList();
      // 很麻烦的创建一系列game的方法↓
      // if(this.games.state.code == 0)
      // {
      //   let gamelist = document.getElementById("gamelist");
      //   for(var i = 0;i < this.games.data.items.length;++i)
      //   {
      //     let gamelist = document.getElementById("gamelist")
      //     let newElement = document.createElement('div')
      //     newElement.id = "newElement";
      //     gamelist.appendChild(newElement);
      //     var g = new Vue(game);
      //     g.data = this.games.data.items[i];
      //     g.$mount("#newElement")
      //     console.log(g)
      //   }
      // }
    },
    data(){
      return{
        games:{
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
          }},
          detail:{}
        }
      },
      methods:{
        getGameList:function()
        {
          const options = {  //这部分因为api接受的不是json格式 所以要先用qs转一下格式
            method: 'GET',
            url: "http://47.100.187.145:8080/api/game_list"
          };
          return axios(options);
        },
        toDetail:function(data)
          {
            this.$emit('det',"gamedetail")
            //this.lucky = this.getRandomGame()
            this.detail = data.name;
            this.$emit('send',this.detail)
          }
      }
    }
</script>

<style>
#gamelist
{
  width: 80%;
  margin-left: 10%;
}
game
{
  height: 80px;
  width: 200px;
  float: left;
}
</style>
