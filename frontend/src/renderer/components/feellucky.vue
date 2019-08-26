<template>
    <div id="feelLucky">
        <div id=functionBar></div>
        <div id="description">
            <img v-bind:src="data.data.image_url" id="gamepic">
            <div id="gametext">{{data.data.desc}}</div>
            <div id="gamename">{{data.data.name}}</div>
        </div>
        <div id="priceData">
            <div class="text">当前价格：{{data.data.current_price}}</div>
            <div class="text">史低价格：{{data.data.lowest_price}}</div>
        </div>
        <div id="addTo_wishlist" @click="addWishList">
            添加至愿望单
        </div>
        <div id="delFrom_wishlist" @click="deleteFromWishList">
            从愿望单删除
        </div>
    </div>

</template>

<script>
    import axios from 'axios'
    import Self from './self'
    export default {
        name: 'feelLucky',
        components:{
        },
        methods:
        {
            addWishList:function()
            {
                let addToWish = {
                    "nickname":"",
                    "gamename":"this.data.data.name",
                    "email_alert":false
                };
                const options = {
                    method: 'POST',
                    data:  JSON.stringify(addToWish),
                    url: "http://47.100.187.145:8080/api/add_wishlist"
                    };
                let element = document.getElementById("addTo_wishlist");
                element.style.backgroundColor = "#96B97D"
                element.innerHTML = "添加成功！"
                return axios(options);
            },
            deleteFromWishList:function()
            {
                console.log(this.data)
                let deleteFromWish = {
                    "nickname":"",
                    "gamename":"this.data.data.name"
                };
                const options = {
                    method: 'POST',
                    headers: { 'content-type': 'application/x-www-form-urlencoded' },
                    data: deleteFromWish,
                    url: "http://47.100.187.145:8080/api/del_wishlist"
                };
                let element = document.getElementById("delFrom_wishlist");
                element.style.backgroundColor = "#96B97D"
                element.innerHTML = "删除成功！"
                return axios(options);
            },
            getRandomGame:function()
            {
                const options = {
                    method: 'GET',
                    url: "http://47.100.187.145:8080/api/random_game"
                }
                return axios(options);
            }
        },
        mounted(){
            var vm = this;
            Self.$on('gamedata',function(val){
                vm.data = val;
            })
            console.log(this.getRandomGame());
        },
        data(){
            return{
                data:{
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
                        "desc": "以廣闊的世界為舞台，無論去哪裡、做什麼，冒險的一切由你決定。狩獵野生動物過活？去消滅魔物？逛盡各處景點？奔跑、游泳、滑翔、攀登，能在廣闊無垠的世界中享受自由自在的冒險。Nintendo Switch的話，可以在家裡的電視上仔細遊玩後繼續外出遊玩，就連遊戲方式都自由自在。\n【故事大綱】被稱為大災厄的災害發生了，海拉魯王國被滅亡了……100年後，主角林克從地下遺跡的長眠中甦醒，在不可思議的聲音引導下踏上大地。"
                    }      
                }
            }
        }
}
</script>

<style>

#description
{
    height: 500px;
    width: 80%;
    margin-top: 40px;
    margin-left: 10%;
    background-color: #EFEFEF;
}
#priceData
{
    height:40px;
    width:80%;
    margin-left: 10%;
    margin-top:10px;
    background-color: #EFEFEF;
}
#gamepic
{
    height: 460px;
    width: 100%;
    position:relative;
}
#gametext
{
    height: 460px;
    width: 100%;
    position: relative;
    top:-460px;
    left: 0;
    background-color: rgba(239,239,239,0.7);
}
#gamename
{
    position: relative;
    top:-460px;
    height: 40px;
    width: 100%;
    line-height: 40px;
    font-size: 28px;
    font-family: '等线','Microsoft Yahei';
    text-align: center;
}
.text
{
    width:50%;
    float: left;
    font-size: 28px;
    font-family: '等线','Microsoft Yahei';
    text-align:center;
    background-color: #EFEFEF;
    height: 40px;
    line-height: 40px;
}
</style>
