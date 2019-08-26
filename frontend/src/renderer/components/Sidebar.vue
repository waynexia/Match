<template>
	<keep-alive>
	<div id="sidebar" >
		<div class="title" id="title">Match</div>
		<button id="hide" @click="hidden"><img src="../assets/settings.png" class="logo"></button>
		<div class="recentFileList">
			<div class="aFile">
				<img src="../assets/settings.png" class="logo">
				<div class="detail">系统设置</div>
			</div>
			<div class="aFile">
				<img src="../assets/settings.png" class="logo">
				<div class="detail">用户设置</div>
			</div>
			<div class="aFile">
				<img src="../assets/settings.png" class="logo">
				<div class="detail">个人信息</div>
			</div>
		</div>
	</div>
	</keep-alive>
</template>

<script>
	export default
	{
		name:'sidebar',
		methods:{
			hidden:function()
			{
				var title = document.getElementById("title");
				var detail = document.getElementsByClassName("detail");
				var sidebar = document.getElementById("sidebar");
				if(detail[0].style.display == "none")
				{
					for(var i = 0;i < detail.length;++i)
						detail[i].style.display = "inline";
					sidebar.style.width = "250px";
					title.style.visibility = "visible";
				}
				else
				{
					for(var i = 0;i < detail.length;++i)
						detail[i].style.display = "none";
					sidebar.style.width = "50px";
					title.style.visibility = "hidden";
				}
				
			}
		}
	}
</script>
<style>
.aFile
{
	color:#2F2F2F;
	height:40px;
	line-height: 40px;
	width:100%;
	float: bottom;
	font-size: 20px;
	font-family: 等线;
	margin-bottom: 8px;
	text-align: left;
}
.aFile:hover
{
	color: rgba(150,185,125);
	background-color:rgba(150,185,125,0.15);
}
.detail
{
	height: 35px;
	width: 100%;
	margin-left: 40px;
	color: black;
	display: none;
}
.logo
{
	padding-left: 10px;
	height: 80%;
	width: calc(height);
	margin-top: 5px;
	float: left;
}
.recentFileList
{
	width:100%;
	height:50%;
	text-align: left;
}
.title
{
	width:100%;
	padding-left:20px;
	height:80px;
	margin-top: 10px;
	font-size: 40px;
	font-family: 等线;
	color: black;
	line-height: 100px;
	float: bottom;
	text-align: left;
	visibility: hidden;
}
#div1
{
	text-align: left;
}
#hide
{
	width: 100%;
	text-align: left;
	height: 40px;
	margin-top: 10px;
	font-family: 等线;
	font-size: 22px;
	color: black;
	background-color: #2F2F2F;
	border: 0px;
	padding-left: 0px;
}
#openOtherFile
{
	color: black;
	width: 100%;
	height: 50px;
	line-height: 50px;
	margin-top: 10px;
	text-align: left;
	font-family: 等线;
	font-size:20px;
	background-color: #686665e0;
}
#openOtherFile:hover
{
	background-color: #594e44;
}
#sidebar
{
	width: 50px;
	height: 100%;
	background-color:#2F2F2F;
	float: left;
	min-height: 100%;
}

</style>