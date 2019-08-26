<template>
	<div class="container">
		<navigation v-on:route="routeTo" class="header"></navigation>
		<gamelist v-on:det="routeTo" v-if="flag=='gamelist'" class="content" v-on:send="Receive"></gamelist>
		<wishlist v-if="flag=='wishlist'" class="content"></wishlist>
		<feellucky v-if="flag=='feellucky'" class="content"></feellucky>
		<gamedetail v-if="flag=='gamedetail'" class="content" v-bind:name="name"></gamedetail>
		<login v-if="flag=='login'" class="content"></login>
	</div>
</template>

<script>
	import navigation from './navigation'
	import gamelist from './gamelist'
	import gamedetail from './gamedetail'
	import wishlist from './wishlist'
	import feellucky from './feellucky'
	import login from './login'
	export default
	{
		name:'page',
		methods:{
			routeTo:function(path)
			{
				this.flag=path;
			},
			Receive:function(name)
			{
				this.name=name;
			}
		},
		components:{
			navigation,gamelist,gamedetail,wishlist,feellucky,login
		},
		data(){
			return{
				flag:"login",
				name
			}
		}
	}
</script>
<style>
.container{
    min-height:100vh;
    display:flex;
    flex-direction: column;
    flex-wrap: wrap;
}
.header{
    height:100px;

}
.content{
    flex:1;
    width:100%;
}
</style>