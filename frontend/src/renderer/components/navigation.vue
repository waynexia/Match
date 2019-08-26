<template>
	<div id="nav">
		<div class="button" @click="GameList">Game List</div>
		<div class="button" @click="WishList">Wish List</div>
		<div class="button" @click="FeelLucky">Feel Lucky</div>
		<div class="button" @click="GameDetail">Detail</div>
	</div>
</template>

<script>
	import axios from 'axios'
	import Self from './self'
	export default
	{
		name:'navigation',
		methods:{
			WishList:function()
			{
				this.$emit('route',"wishlist")
			},
			GameList:function()
			{
				this.$emit('route',"gamelist")
			},
			GameDetail:function()
			{
				this.$emit('route',"gamedetail")
			},
			FeelLucky:function()
			{
				this.$emit('route',"feellucky")
				this.lucky = this.getRandomGame()
				Self.$emit('gamedata',this.lucky)
			},
			getRandomGame:function()
			{
				const options = {  //这部分因为api接受的不是json格式 所以要先用qs转一下格式
	            	method: 'GET',
	            	url: "47.100.187.145:8080/api/game_list"
	          	};
	          	return axios(options);
			}
		},
		data(){
			return{
				lucky:{}
			}
		}
	}
</script>
<style>
.button
{
	margin-top: 50px;
	width:25%;
	height:50px;
	background-color: #EFEFEF;
	text-align: center;
	line-height: 50px;
	float: left;
}
.button:hover
{
	background-color: #8F8F8F;
}

#nav
{
	width: 80%;
	margin-left: 10%;
}
</style>