<template>
    <div class="game" @click="sendData">
        <img v-bind:src="data.image_url" id="gamepics" @click="sendData">
        <div id="info">
            <div class="gameListText" id="gameNameText">{{data.name}}</div>
            <div class="gameListText">Original price:￥{{data.original_price}}</div>
            <div class="gameListText" >Current price:￥{{data.current_price}}</div>
        </div>
    </div>

</template>

<script>
    export default {
        props:[
            "data"
        ],
        name: 'game',
        components:{
        },
        methods:{
            sendData:function()
            {
                this.$emit('send',this.data)
            }
        }
}
</script>

<style>
.gameListText
{
    height: 50px;
    text-align:center;
    font-size:14px;
    line-height: 50px;
}
.game
{
    display: flex;
    flex-direction: row;
    height: 150px;
    float:left;
    margin: 20px;
    width:40%;
    margin-left: 5%;
    margin-right: 5%;
    margin-top: 20px;
}
#gamepics
{
    height: 150px;
    width: 150px;
}
#gameNameText
{
    height: 50px;
    line-height: 50px;
    overflow: hidden;
}
#info
{
    flex:1;
    display: flex;
    flex-direction: column;
}

</style>
