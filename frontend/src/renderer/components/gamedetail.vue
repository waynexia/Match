<template>
    <div id="gameDetail">
        <div id=functionBar></div>
        <div id="description">
            <img v-bind:src="data.image_url" id="gamepic">
            <div id="gametext">{{data.desc}}</div>
            <div id="gamename">{{name}}</div>
        </div>
        <div id="priceData">
            <div class="text">当前价格：{{data.current_price}}</div>
            <div class="text">史低价格：{{data.lowest_price}}</div>
        </div>
        <div id="addTo_wishlist" @click="addToWishList">
            添加至愿望单
        </div>
        <div id="delFrom_wishlist" @click="deleteFromWishList">
            从愿望单删除
        </div>
    </div>

</template>

<script>
    import Self from './self'
    import axios from 'axios'
    export default {
        name: 'gameDetail',
        components:{
        },
        created(){
            this.data = {};
            let that = this;
            this.getDetail(that);
        },
        methods:
        {
            addToWishList:function()
            {
                let addToWish = {
                    "nickname":"1",
                    "gamename":"1",
                    "email_alert":false
                };
                const options = {
                    method: 'POST',
                    data: JSON.stringify(addToWish),
                    url: "http://47.100.187.145:8080/api/add_wishlist"
                };
                let element = document.getElementById("addTo_wishlist");
                element.style.backgroundColor = "#96B97D"
                element.innerHTML = "添加成功！"
                return axios(options);
            },
            deleteFromWishList:function()
            {
                let deleteFromWish = {
                    "nickname":"",
                    "gamename":this.data.name
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
            getDetail:function(t)
            {
                let d = {
                    "gamename":t.name
                }
                const options = {
                    method:'POST',
                    data:  JSON.stringify(d),
                    url: "http://47.100.187.145:8080/api/get_detail"
                }
                return axios(options)
            }
        },
        props:[
            "name"
        ],
        data(){
            return{
                data:{}
            }
        },
        computed:{
            getName(){
                console.log(this.name)
                return this.name;
            }
        }
            
}
</script>

<style>
#addTo_wishlist
{
    width: 40%;
    height: 40px;
    line-height: 40px;
    font-size: 16px;
    font-family: "等线";
    margin-left: 10%;
    text-align: center;
    background-color: #EFEFEF;
    margin-top:10px;
    float: left;
}
#delFrom_wishlist
{
    width:40%;
    height: 40px;
    line-height: 40px;
    font-size: 16px;
    font-family: "等线";
    text-align: center;
    background-color: #EFEFEF;
    float:left;
    margin-top:10px;
}
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
