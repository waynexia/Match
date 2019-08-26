<template>
  <div id="wishlist">
    <game v-for="item in games.data.items" v-bind:data="item"></game>
  </div>

</template>

<script>
  import game from './game'
  import axios from 'axios'
  import Vue from 'vue'
  export default {
    name: 'gamelist',
    components:{
      game
    },
    mounted(){
      this.games = this.getWishList();
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
          }}
        }
      },
      methods:{
        getWishList:function()
        {
          const options = {  //这部分因为api接受的不是json格式 所以要先用qs转一下格式
            method: 'GET',
            data:{
              "nickname":'Alison'
            },
            url: "http://47.100.187.145:8080/api/get_wishlist"
          };
        return axios(options);
        }
    }
  }
</script>

<style>
#wishlist
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
